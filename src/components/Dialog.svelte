<script lang="ts">
    import { fade, scale } from "svelte/transition";
    import current from "../lib/dialog.svelte";
    import { untrack } from "svelte";

    let dialog = $derived(current());

    let input: string | null = $state(null);

    $effect(() => {
        if (dialog || !dialog) {
            untrack(() => {
                input = null;
            });
        }
    });
</script>

{#if dialog}
    <div
        class="z-20 fixed top-0 left-0 w-full h-full flex flex-col items-center justify-center bg-[rgba(0,0,0,0.5)]"
        transition:fade
    >
        <div
            class="overflow-hidden bg-yellow-100 flex flex-col shadow-xl"
            in:scale
            out:scale
        >
            {#if dialog.title}
                <div class="bg-yellow-400 py-2 px-3 min-w-100">
                    {dialog.title}
                </div>
            {/if}
            {#if dialog.message}
                <div class="py-2 px-3 whitespace-pre-line">
                    {dialog.message}
                </div>
            {/if}
            {#if dialog.input}
                <div class="py-2 px-2">
                    <input
                        type="text"
                        class="input"
                        bind:value={input}
                        placeholder="Type here..."
                    />
                </div>
            {/if}
            <div class="bg-yellow-20 bg-amber-200">
                {#each dialog.buttons ?? [] as button}
                    <button
                        class="btn py-2 px-3 font-bold hover:bg-amber-300 transition"
                        onclick={() =>
                            dialog.resolve({
                                button: button.id,
                                input: input,
                            })}
                    >
                        {button.label}
                    </button>
                {/each}
            </div>
        </div>
    </div>
{/if}
