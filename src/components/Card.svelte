<script lang="ts">
    import Icon from "@jamescoyle/svelte-icon";
    import { mdiInformation } from "@mdi/js";
    import { mdiAlert } from "@mdi/js";
    import { mdiAlertOctagon } from "@mdi/js";
    import type { Snippet } from "svelte";

    let {
        style,
        title,
        class: className,
        children,
    }: {
        style: "info" | "warning" | "danger";
        title: string;
        class?: string;
        children: Snippet;
    } = $props();

    let bgColor = $derived(
        { info: "bg-sky-100", warning: "bg-amber-100", danger: "bg-red-100" }[
            style
        ],
    );
    let iconColor = $derived(
        {
            info: "--color-sky-500",
            warning: "--color-amber-500",
            danger: "--color-red-500",
        }[style],
    );
    let iconPath = $derived(
        { info: mdiInformation, warning: mdiAlert, danger: mdiAlertOctagon }[
            style
        ],
    );
</script>

<div class="bg-zinc-50 shadow {className}">
    <div class="py-2 px-3 {bgColor} flex gap-2">
        <Icon path={iconPath} type="mdi" color="var({iconColor})"></Icon>
        {title}
    </div>
    <div class="py-2 px-3">
        {@render children()}
    </div>
</div>
