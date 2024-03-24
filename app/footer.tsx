'use client';
export default function Footer() {
    return (
        <div className={ `bottom-0 mt-5 content-center h-fit bg-slate-800/95` }>
            <p className={ `text-center my-2` }>©2024-{ new Date().getFullYear() } <a
                href={ 'https://github.com/lumither' }>Lumither Tao</a></p>
        </div>
    );
}
