'use client';
import React, { useEffect, useRef } from 'react';

// WebGL-based rewrite of the grid/hover effect originally implemented with many DOM nodes.
// Key design goals:
//  - zero per-cell DOM; a single full-screen draw with a fragment shader
//  - hover color derived deterministically from a small palette
//  - optional CSS transform to match the original perspective/skew
//  - high-DPI aware; resizes with container

export default function BoxesGL({
    className,
    style,
    rows = 250,
    cols = 200,
    lineWidth = 1.0, // device pixels
    transformLikeOriginal = true
}: {
    className?: string
    style?: React.CSSProperties
    rows?: number
    cols?: number
    lineWidth?: number
    transformLikeOriginal?: boolean
}) {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const rafRef = useRef<number | null>(null);
    const mouseRef = useRef<{ x: number; y: number } | null>(null);

    useEffect(() => {
        const canvas = canvasRef.current!;
        const gl = canvas.getContext('webgl2', { antialias: false, premultipliedAlpha: true });
        if (!gl) {
            console.warn('WebGL2 not available');
            // Fail closed; nothing to do if WebGL2 is unavailable
            return;
        }

        // Create program (fullscreen triangle)
        const vs = gl.createShader(gl.VERTEX_SHADER)!;
        gl.shaderSource(
            vs,
            `#version 300 es\n
      const vec2 verts[3] = vec2[3](
        vec2(-1.0, -1.0),
        vec2(3.0, -1.0),
        vec2(-1.0, 3.0)
      );
      void main(){
        gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
      }
    `
        );
        gl.compileShader(vs);

        const fs = gl.createShader(gl.FRAGMENT_SHADER)!;
        gl.shaderSource(
            fs,
            `#version 300 es\n
      precision highp float;
      out vec4 o;

      uniform vec2 u_resolution;      // canvas size in device pixels
      uniform vec2 u_mouse;           // mouse in device pixels, (-1,-1) if outside
      uniform float u_time;           // seconds
      uniform float u_rows;
      uniform float u_cols;
      uniform float u_lineWidth;      // in device pixels

      // Border color and plus sign color (matches tailwind slate-700-ish)
      const vec3 BORDER = vec3(0.22, 0.27, 0.33);
      const vec3 PLUS   = vec3(0.22, 0.27, 0.33);

      // A small pastel palette like the original
      vec3 palette(int idx){
        // 9-color palette
        if(idx==0) return vec3(0.576,0.773,0.992);
        if(idx==1) return vec3(0.976,0.659,0.831);
        if(idx==2) return vec3(0.525,0.937,0.675);
        if(idx==3) return vec3(0.992,0.878,0.278);
        if(idx==4) return vec3(0.988,0.648,0.648);
        if(idx==5) return vec3(0.847,0.706,0.996);
        if(idx==6) return vec3(0.576,0.773,0.992);
        if(idx==7) return vec3(0.647,0.706,0.988);
        return             vec3(0.769,0.710,0.992);
      }

      // Hash helpers for stable per-cell randomness
      float hash11(float x){
        x = fract(x * 0.1031);
        x *= x + 33.33;
        return fract(1e4 * x * (x + 0.33));
      }
      float hash21(vec2 p){
        vec3 p3 = fract(vec3(p.xyx) * 0.1031);
        p3 += dot(p3, p3.yzx + 33.33);
        return fract((p3.x + p3.y) * p3.z);
      }

      void main(){
        vec2 frag = gl_FragCoord.xy;               // device pixels
        vec2 gridSize = vec2(u_cols, u_rows);
        vec2 cellSize = u_resolution / gridSize;   // pixels per cell

        // Compute integer cell coordinates and local position within cell [0,1)
        vec2 uvGrid = frag / u_resolution * gridSize;
        vec2 cellId = floor(uvGrid);
        vec2 cellUV = fract(uvGrid);

        // Grid lines: draw top+right borders to mimic original (border-t, border-r, border-l on first col)
        float lw = u_lineWidth / max(cellSize.x, 1.0); // convert to relative width in [0,1] axis
        float lwY = u_lineWidth / max(cellSize.y, 1.0);

        float lineTop  = step(1.0 - lwY, cellUV.y);
        float lineRight= step(1.0 - lw , cellUV.x);
        float lineLeft = step(cellUV.x, lw) * step(0.5, step(0.0, cellId.x)); // left border for all cols via small bias

        float lineMask = max(lineTop, max(lineRight, lineLeft));

        // Determine hovered cell index
        vec2 hoverCell = vec2(-1.0);
        bool hovering = u_mouse.x >= 0.0 && u_mouse.y >= 0.0;
        if(hovering){
          vec2 hoverUVGrid = u_mouse / u_resolution * gridSize;
          hoverCell = floor(hoverUVGrid);
        }
        bool isHover = hovering && all(equal(hoverCell, cellId));

        // Background: transparent unless hovered. On hover, fill with palette color chosen by cell hash.
        vec3 bg = vec3(0.0);
        float alpha = 0.0;
        if(isHover){
          // Stable color per cell, with slight time wobble to avoid banding on long hovers
          float h = hash21(cellId + vec2(floor(u_time*0.25)));
          int idx = int(floor(h * 9.0)) % 9;
          bg = palette(idx);
          alpha = 1.0;
        }

        // Plus sign for even-even cells: thin cross centered
        bool evenEven = mod(cellId.x, 2.0) < 0.5 && mod(cellId.y, 2.0) < 0.5;
        float plusThicknessX = 0.5 * u_lineWidth / max(cellSize.x, 1.0);
        float plusThicknessY = 0.5 * u_lineWidth / max(cellSize.y, 1.0);
        float cross = 0.0;
        if(evenEven){
          float vert = step(0.5 - plusThicknessX, abs(cellUV.x - 0.5));
          float hori = step(0.5 - plusThicknessY, abs(cellUV.y - 0.5));
          cross = 1.0 - max(vert, hori);
        }

        vec3 color = mix(bg, BORDER, max(lineMask, 0.0));
        color = mix(color, PLUS, cross);
        float a = max(alpha, max(lineMask, cross));

        o = vec4(color, a);
      }
    `
        );
        gl.compileShader(fs);

        const prog = gl.createProgram()!;
        gl.attachShader(prog, vs);
        gl.attachShader(prog, fs);
        gl.linkProgram(prog);

        // Check compile/link for diagnostics (silent in prod; could surface to console in dev)
        if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
            console.warn('WebGL program link error:', gl.getProgramInfoLog(prog));
        }

        gl.useProgram(prog);

        const uResolution = gl.getUniformLocation(prog, 'u_resolution');
        const uMouse = gl.getUniformLocation(prog, 'u_mouse');
        const uTime = gl.getUniformLocation(prog, 'u_time');
        const uRows = gl.getUniformLocation(prog, 'u_rows');
        const uCols = gl.getUniformLocation(prog, 'u_cols');
        const uLineWidth = gl.getUniformLocation(prog, 'u_lineWidth');

        // Single draw call state
        gl.disable(gl.DEPTH_TEST);
        gl.disable(gl.CULL_FACE);
        gl.viewport(0, 0, canvas.width, canvas.height);

        // Resize handling (devicePixelRatio-aware)
        const resize = () => {
            const dpr = Math.max(window.devicePixelRatio || 1, 1);
            const { clientWidth, clientHeight } = canvas;
            const w = Math.max(1, Math.floor(clientWidth * dpr));
            const h = Math.max(1, Math.floor(clientHeight * dpr));
            if (canvas.width !== w || canvas.height !== h) {
                canvas.width = w;
                canvas.height = h;
                gl.viewport(0, 0, w, h);
            }
        };

        const observer = new ResizeObserver(resize);
        observer.observe(canvas);
        resize();

        // Mouse tracking in device pixels
        const onMove = (e: PointerEvent) => {
            const rect = canvas.getBoundingClientRect();
            const dpr = Math.max(window.devicePixelRatio || 1, 1);
            const x = (e.clientX - rect.left) * dpr;
            const y = (rect.bottom - e.clientY) * dpr; // flip Y to GL coords
            mouseRef.current = { x, y };
        };
        const onLeave = () => {
            mouseRef.current = null;
        };
        canvas.addEventListener('pointermove', onMove);
        canvas.addEventListener('pointerleave', onLeave);

        const t0 = performance.now();
        const tick = () => {
            const t = (performance.now() - t0) / 1000;
            gl.useProgram(prog);
            gl.uniform2f(uResolution, canvas.width, canvas.height);
            const m = mouseRef.current;
            if (m) gl.uniform2f(uMouse, m.x, m.y);
            else gl.uniform2f(uMouse, -1, -1);
            gl.uniform1f(uTime, t);
            gl.uniform1f(uRows, rows);
            gl.uniform1f(uCols, cols);
            gl.uniform1f(uLineWidth, lineWidth);

            gl.drawArrays(gl.TRIANGLES, 0, 3);
            rafRef.current = requestAnimationFrame(tick);
        };
        tick();

        return () => {
            if (rafRef.current) cancelAnimationFrame(rafRef.current);
            observer.disconnect();
            canvas.removeEventListener('pointermove', onMove);
            canvas.removeEventListener('pointerleave', onLeave);
            gl.deleteProgram(prog);
            gl.deleteShader(vs);
            gl.deleteShader(fs);
        };
    }, [ rows, cols, lineWidth ]);

    return (
        <div
            className={ [
                'absolute -top-1/4 left-1/4 z-0 flex h-full w-full -translate-x-1/2 -translate-y-1/2 p-4',
                className || ''
            ].join(' ') }
            style={ style }
        >
            <canvas
                ref={ canvasRef }
                style={ {
                    width: '100%',
                    height: '100%',
                    // apply the same visual transform as the original DOM version
                    transform: transformLikeOriginal
                        ? 'translate(-40%,-60%) skewX(-48deg) skewY(14deg) scale(0.675) rotate(0deg) translateZ(0)'
                        : undefined
                } }
            />
        </div>
    );
}
