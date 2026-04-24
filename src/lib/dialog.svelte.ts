type Dialog = {
    title: string,
    message: string,
    input?: boolean,
    buttons: Button[]
}

type ResolutionData = {
    button: string,
    input: string | null,
}

type AwaitDialog = Dialog & {
    resolve: (data: ResolutionData) => void
};

type Button = {
    id: string,
    label: string,
}

const dialogs: AwaitDialog[] = $state([]);
const _currentDialog = $derived(dialogs[dialogs.length - 1]);

export default function current(): AwaitDialog | undefined {
    return _currentDialog;
}

export async function pushDialog(dialog: Dialog): Promise<ResolutionData> {
    let data = await new Promise<ResolutionData>((resolve) => {
        let ad: AwaitDialog = dialog as AwaitDialog;
        ad.resolve = resolve;
        dialogs.push(ad);
    });
    dialogs.splice(0, 1);
    return data;
}

export async function pushConfirm(title: string, message: string): Promise<boolean> {
    let result = await pushDialog({
        title: title,
        message: message,
        buttons: [
            { id: "yes", label: "Yes" },
            { id: "no", label: "No" },
        ]
    })
    return result.button === "yes"
}

export async function pushPrompt(title: string, message: string): Promise<string | null> {
    let result = await pushDialog({
        title: title,
        message: message,
        input: true,
        buttons: [
            { id: "ok", label: "Ok" },
            { id: "cancel", label: "Cancel" },
        ]
    })
    if (result.button === "ok") {
        return result.input;
    }
    return null;
}
