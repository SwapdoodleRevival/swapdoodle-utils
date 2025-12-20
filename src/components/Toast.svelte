<script lang="ts">
    import { scale } from "svelte/transition";
    import toasts from "../lib/toast.svelte";
    import Card from "./Card.svelte";
</script>

<div
    class="z-10 fixed top-0 right-0 w-full flex flex-col items-end p-4 box-border pointer-events-none"
>
    {#each toasts().entries() as [key, t] (key)}
        <div transition:scale class="shadow-2xl mb-2">
            <Card style={t.type} title={t.title} class="flex-hidden">
                {t.message}
            </Card>
        </div>
    {/each}
</div>
