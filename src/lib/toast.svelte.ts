import { SvelteMap } from "svelte/reactivity";

type Toast = {
    type: "info" | "warning" | "danger",
    title: string,
    message: string
}

type BaseToast = Omit<Toast, "type">;

const toasts: Map<number, Toast> = new SvelteMap();

export function warn(toast: BaseToast) {
    pushToast(Object.assign({ type: "warning" } as Toast, toast))
}

export function error(toast: BaseToast) {
    pushToast(Object.assign({ type: "danger" } as Toast, toast))
}

export function info(toast: BaseToast) {
    pushToast(Object.assign({ type: "info" } as Toast, toast))
}

export function pushToast(toast: Toast) {
    let key = +new Date();
    toasts.set(key, toast);
    setTimeout(() => {
        toasts.delete(key);
    }, 5000);
}

export default function all() {
    return toasts;
}