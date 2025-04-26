<script lang="ts">
    let {
        onfileaccepted,
    }: {
        onfileaccepted: (content: File) => any;
    } = $props();
    let fileInput: HTMLInputElement = $state()!;

    function fileOpen() {
        fileInput.click();
    }

    function fileSelected(e: Event) {
        let file = (e.target as HTMLInputElement | null)?.files?.[0];
        if (file) {
            onfileaccepted(file);
        }
    }
</script>

<div class="open-file">
    <button class="target" onclick={fileOpen}>
        <h1>Swapdoodle File Viewer</h1>
        <p>Click here or drag a file onto this page to open it</p>
    </button>
    <a
        href="https://github.com/Silver-Volt4/swapdoodle-utils/wiki/Dumping-letter-files"
        target="_blank"
        class="hint"
    >
        Where do I find Letter/Doodle files?
    </a>
</div>
<input bind:this={fileInput} type="file" onchange={fileSelected} />

<style>
    .open-file {
        display: flex;
        flex-direction: column;
        align-items: center;
        height: 100vh;
        justify-content: center;
    }

    .target {
        padding: 2em;
        border: dashed 4px green;
        border-radius: 2em;
        background: none;
        cursor: pointer;
        font-size: 125%;
    }

    input {
        display: none;
    }

    a.hint {
        margin-top: 1em;
    }
</style>
