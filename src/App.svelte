<script lang="ts">
    import { parse_letter, type Letter } from "./lib/parsing/parsing";
    import toast from "./lib/toast.svelte";
    import OpenFile from "./pages/OpenFile.svelte";
    import Toast from "./components/Toast.svelte";
    import ViewFile from "./pages/ViewFile.svelte";

    function dragOver(e: Event) {
        e.preventDefault();
    }

    function drop(e: DragEvent) {
        e.preventDefault();

        let file = e.dataTransfer?.files[0];
        if (file) {
            readFile(file);
        }
    }

    function readFile(file: File) {
        let reader = new FileReader();
        reader.onload = (readerEvent) => {
            let content = readerEvent.target?.result as ArrayBuffer | null;
            if (!content) {
                return;
            }
            letterData = new Uint8Array(content);
            try {
                letter = parse_letter(letterData);
            } catch {
                toast.pushToast({
                    title: "Error reading file",
                    message: "This file does not seem to be a Swapdoodle Letter."
                })
            }
        };

        reader.readAsArrayBuffer(file);
    }

    let letterData: Uint8Array | undefined = $state();
    let letter: Letter | undefined = $state();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="root" ondragover={dragOver} ondrop={drop}>
    {#if !(letter && letterData)}
        <OpenFile onfileaccepted={(file) => readFile(file)} />
    {:else}
        <ViewFile {letter} {letterData}></ViewFile>
    {/if}
    <Toast></Toast>
</div>

<style>
    .root {
        min-height: 100vh;
    }
</style>
