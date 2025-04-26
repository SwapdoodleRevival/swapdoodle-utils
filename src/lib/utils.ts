import toast from "./toast.svelte";

export function invokeDownload(data: Uint8Array | Blob, filename: string) {
    let blob: Blob;
    if (data instanceof Uint8Array) {
        blob = new Blob([data], {
            type: "application/octet-stream",
        });
    } else {
        blob = data;
    }
    let downloadUrl = URL.createObjectURL(blob);
    let a = document.createElement("a");
    a.download = filename;
    a.href = downloadUrl;
    a.click();
    URL.revokeObjectURL(downloadUrl);
}

export class LoudError extends Error {
    constructor(message: string, title?: string, options?: ErrorOptions) {
        super(message, options);
        toast.error({
            title: title ?? "Error",
            message: message
        })
    }
}

export class CanvasContextCreationError extends LoudError {
    constructor() {
        super("Failed to create canvas context (see dev console).");
    }
}