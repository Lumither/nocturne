export function todo(message: string = 'Not yet implemented'): never {
    throw new Error(`TODO: ${ message }`);
}