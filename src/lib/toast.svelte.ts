import { SvelteMap } from "svelte/reactivity";

type Toast = {
    type: "info" | "warn" | "error",
    title: string,
    message: string
}

type TypedToast = Omit<Toast, "type">;

const toasts: Map<number, Toast> = new SvelteMap();

function warn(toast: TypedToast) {
    pushToast(Object.assign({ type: "warn" } as Toast, toast))
}

function error(toast: TypedToast) {
    pushToast(Object.assign({ type: "error" } as Toast, toast))
}

function info(toast: TypedToast) {
    pushToast(Object.assign({ type: "info" } as Toast, toast))
}

function pushToast(toast: Toast) {
    let key = +new Date();
    toasts.set(key, toast);
    setTimeout(() => {
        toasts.delete(key);
    }, 5000);
}

export default {
    get toasts() {
        return toasts;
    },
    info,
    warn,
    error,
    pushToast
};