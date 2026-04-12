import { OpenedFile } from "./libdoodle/libdoodle.svelte";
import { warn } from "./toast.svelte";

export let files: OpenedFile[] = $state([]);
let _currentFile: OpenedFile | null = $state.raw(null);

export function askForFile(): Promise<FileList | null> {
    let fileInput = document.createElement("input")
    fileInput.type = "file"

    return new Promise((resolve, reject) => {
        const success = (e: Event) => {
            fileInput.removeEventListener("cancel", failure);
            resolve((e.target as HTMLInputElement | null)?.files ?? null);
        };
        const failure = (e: Event) => {
            fileInput.removeEventListener("change", success);
            reject("No file was selected");
        }
        fileInput.addEventListener("change", success, { once: true })
        fileInput.addEventListener("cancel", failure, { once: true })
        fileInput.click();
    })
}

export async function openNewFile(file: File | Uint8Array<ArrayBufferLike>, name: string | null = null) {
    try {
        let bpk1File: OpenedFile;
        if (file instanceof File) {
            name ??= file.name;
            bpk1File = await OpenedFile.readFile(file);
        }
        else {
            bpk1File = await OpenedFile.fromBytes(name ?? "new-file.bpk1", file);
        }

        files.push(bpk1File);
        setCurrentFile(bpk1File);
    } catch (e) {
        let message = (e as Partial<Error>)?.message;
        warn({
            title: "Error reading file",
            message: message ?? "Unknown error",
        });
    }
}

export function getCurrentFile() {
    return _currentFile
}

export function setCurrentFile(newFile: OpenedFile) {
    _currentFile = newFile
}

export function closeCurrentFile() {
    let index = files.indexOf(_currentFile!);
    if (index === -1) {
        return;
    }

    files.splice(index, 1);
    if (files.length === 0) {
        _currentFile = null;
        return;
    }
    if (files.length <= index) {
        index -= 1;
    }

    _currentFile = files[index];
}
