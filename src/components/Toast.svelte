<script lang="ts">
    import { slide } from "svelte/transition";
    import toast from "../lib/toast.svelte";
</script>

<div class="toasts">
    {#each toast.toasts.entries() as [key, t] (key)}
        <div class="toast {t.type}" transition:slide>
            <div class="title">
                {t.title}
            </div>
            <div class="message">
                {t.message}
            </div>
        </div>
    {/each}
</div>

<style>
    .toasts {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        display: flex;
        align-items: center;
        flex-direction: column;
        padding: 2em 1em;
        box-sizing: border-box;
        pointer-events: none;
    }

    .toast {
        border-radius: 1em;
        overflow: hidden;
        background-color: white;
        display: flex;
        flex-direction: column;
        box-shadow:
            0 3px 6px rgba(0, 0, 0, 0.16),
            0 3px 6px rgba(0, 0, 0, 0.23);
        margin-bottom: 1em;
    }

    .toast .title {
        padding: 0.5em 1em;
        font-weight: bold;
    }

    .toast.warn .title {
        background-color: gold;
    }

    .toast.error .title {
        background-color: red;
    }

    .toast.info .title {
        background-color: lightskyblue;
    }

    .toast .message {
        padding: 1em;
        white-space: pre-line;
    }
</style>
