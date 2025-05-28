<script lang="ts">
    import { LetterFile } from "./lib/libdoodle/libdoodle.svelte";
    import { warn } from "./lib/toast.svelte";
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

    async function readFile(file: File) {
        try {
            letter = await LetterFile.readFile(file);
        } catch (e) {
            let message = (e as Partial<Error>)?.message;
            warn({
                title: "Error reading file",
                message: message ?? "Unknown error",
            });
        }
    }

    let letter: LetterFile | undefined = $state();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="root" ondragover={dragOver} ondrop={drop}>
    {#if !letter}
        <OpenFile onfileaccepted={(file) => readFile(file)} />
    {:else}
        <ViewFile file={letter}></ViewFile>
    {/if}
    <Toast></Toast>
</div>

<style>
    .root {
        min-height: 100vh;
    }
</style>
