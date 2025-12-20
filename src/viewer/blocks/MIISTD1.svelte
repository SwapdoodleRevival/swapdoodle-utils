<script lang="ts">
    import BlobImage from "../../components/BlobImage.svelte";
    import Card from "../../components/Card.svelte";
    import { parse_mii_data } from "../../lib/libdoodle/libdoodle.svelte";
    import type {
        BPK1Block,
        BPK1File,
    } from "../../lib/libdoodle/libdoodle.svelte";

    let {
        file,
        block,
    }: {
        file: BPK1File;
        block: BPK1Block;
    } = $props();

    let miidata = $derived(parse_mii_data(block));
</script>

<Card style="info" title="About THUMB2" class="mb-2">
    MIISTD1 blocks contain data about the sender's Mii.
</Card>

<img class="bg-white rounded-full" src={miidata.url} alt="Sender's Mii"/>
<div class="text-xs my-2">
    <b>Note:</b> Mii images are rendered using Nintendo's Mii Studio API.<br>
    If the image does not appear, you are either offline, or Mii Studio is down.
</div>

<div>
    Name: <b>{miidata.name}</b>
</div>
{#if miidata.author_name}
    <div>
        Author: <b>{miidata.author_name}</b>
    </div>
{/if}
