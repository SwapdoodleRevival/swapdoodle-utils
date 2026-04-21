<script lang="ts">
    import Toast from "./components/Toast.svelte";
    import ViewFile from "./viewer/ViewFile.svelte";
    import Dialog from "./components/Dialog.svelte";
    import {
        askForFile,
        files,
        openNewFile,
        getCurrentFile,
        setCurrentFile,
        closeCurrentFile,
    } from "./lib/files.svelte";
    import { mdiOpenInNew } from "@mdi/js";
    import Icon from "@jamescoyle/svelte-icon";
    import { info, warn } from "./lib/toast.svelte";
    import { onMount } from "svelte";
    import { LoudError } from "./lib/utils";

    onMount(() => {
        window.removeEventListener("hashchange", handleHashChanged);
        handleHashChanged();
    });

    function listenForHashChanged() {
        window.addEventListener("hashchange", handleHashChanged, {
            once: true,
        });
    }

    function handleHashChanged() {
        try {
            let hash = location.hash.substring(1);
            location.hash = "";
            processHash(hash);
        } finally {
            setTimeout(listenForHashChanged, 1);
        }
    }

    function processHash(hash: string) {
        let params = new URLSearchParams(hash);

        const action = params.get("action");

        if (action === "netload") {
            console.log("netload");
            const url = params.get("url");
            if (!url) {
                throw new LoudError(
                    'Command "netload" requires parameter "url", which was not provided',
                );
            }
            const label = params.get("label");
            loadFromNetwork(url, label);
        }
    }

    function dragOver(e: Event) {
        e.preventDefault();
    }

    async function drop(e: DragEvent) {
        e.preventDefault();

        let files = e.dataTransfer?.files;
        if (!files) return;
        for (let file of files) {
            if (file) {
                await openNewFile(file);
            }
        }
    }

    async function loadFromNetwork(url: string, label: string | null = null) {
        info({ title: "Netload", message: "Opening file from URL address." });
        let f: Response;
        try {
            f = await fetch(url);
        } catch (error) {
            throw new LoudError(
                (error as Error).message,
                "Load from network failed!",
            );
        }
        let blob = await f.blob();
        openNewFile(await blob.bytes(), label);
    }

    async function fileOpen() {
        try {
            let files = await askForFile();
            for (let file of files ?? []) {
                await openNewFile(file);
            }
        } catch (e) {
            warn({
                title: "Error opening file",
                message: String(e),
            });
        }
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="w-dvw h-dvh flex flex-col" ondragover={dragOver} ondrop={drop}>
    <!-- Open files -->
    <div class="flex w-full bg-yellow-700 text-white shadow-md z-10">
        <button
            onclick={fileOpen}
            class="btn p-2 pr-3 transition flex shrink-0 gap-2 bg-yellow-700 hover:bg-yellow-900 border-e-2 border-yellow-950"
        >
            <Icon path={mdiOpenInNew} type="mdi" color="white"></Icon>

            Open file
        </button>
        <div class="flex overflow-x-scroll grow">
            {#each files as file}
                <button
                    onclick={() => setCurrentFile(file)}
                    class="btn p-2 transition {getCurrentFile() === file
                        ? 'border-solid border-b-2 border-white bg-yellow-600'
                        : 'hover:bg-yellow-900'}"
                >
                    {file.fileName}
                </button>
            {/each}
        </div>
    </div>

    <!-- Viewer -->
    <div class="flex flex-col grow overflow-y-auto">
        {#if getCurrentFile()}
            <ViewFile file={getCurrentFile()!} onclose={closeCurrentFile}
            ></ViewFile>
        {:else}
            <button
                class="w-full grow flex flex-col justify-center self-center items-center"
                onclick={fileOpen}
            >
                <h1 class="font-bold text-6xl">Swapdoodle Utils</h1>
                <p class="text-lg">
                    Click here or drag a file onto this page to open it
                </p>
            </button>
        {/if}
    </div>
</div>

<Toast></Toast>
<Dialog></Dialog>
