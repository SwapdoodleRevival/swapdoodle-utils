<script lang="ts">
    import HexEditor from "js-hex-editor/dist-svelte/HexEditor.svelte";
    import type { BackendBPK1Block } from "../lib/libdoodle/libdoodle.svelte";

    let {
        block,
    }: {
        block: BackendBPK1Block;
    } = $props();

    $effect(() => {
        if (block) {
            displayed = false;
        }
    });

    let displayed = $state(false);
</script>

<div>
    {#if displayed}
        <div class="editor mt-2">
            <HexEditor
                data={block.data.buffer as ArrayBuffer}
                height="400px"
                width="100%"
            />
        </div>
    {/if}
    <button class="btn std mt-2" onclick={() => (displayed = !displayed)}>
        {#if displayed}
            Hide raw bytes
        {:else}
            Show raw bytes
        {/if}
    </button>
</div>

<style>
    @reference "tailwindcss";

    .editor {
        :global(.number),
        :global(.ascii) {
            @apply font-mono;
        }

        :global(.header-offset),
        :global(.header-data),
        :global(.header-ascii) {
            @apply mb-2;

            :global(select) {
                @apply cursor-pointer bg-yellow-200 py-2 px-3 shadow hover:bg-yellow-300 active:bg-yellow-400 transition;
            }
        }

        > :global(main) {
            @apply border-none;

            > :global(header) {
                @apply bg-none border-none shadow-none;
            }

            > :global(svelte-virtual-list-viewport) {
                @apply bg-white shadow;
            }

            > :global(footer) {
                @apply bg-zinc-300 border-none py-2 px-3 mt-2 shadow;
            }
        }
    }
</style>
