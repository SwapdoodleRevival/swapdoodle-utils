<script lang="ts">
    import { LetterFile } from "../lib/parsing/parsing.svelte";
    import BlobImage from "../components/BlobImage.svelte";
    import Doodle from "../components/Doodle.svelte";

    let {
        file,
    }: {
        file: LetterFile;
    } = $props();

    let letter = $derived(file.letter);
</script>

{#snippet bpk1BlockList(blocksMap: Map<string, Uint8Array[]>)}
    <div class="sections">
        {#each blocksMap.entries() as [name, blocks]}
            {#if blocks.length <= 1}
                <button onclick={() => file.downloadBpkBlock(name, 0)}>
                    {name}
                </button>
            {:else}
                <div class="btn noclick">
                    <span style="margin-right: 0.5em;">
                        {name}
                    </span>
                    {#each blocks as block, index}
                        <button
                            onclick={() => file.downloadBpkBlock(name, index)}
                        >
                            #{index + 1}
                        </button>
                    {/each}
                </div>
            {/if}
        {/each}
    </div>
{/snippet}

<div class="file">
    <div class="header">
        <div class="title">Swapdoodle file viewer</div>
        <button onclick={() => file.downloadDecompressedBpk("letter.bpk")}>
            Save letter (decompressed)
        </button>
    </div>

    <div class="card">
        <div class="card-header">BPK1 sections</div>

        <p>
            Clicking any of these buttons will download the binary data for that
            section
        </p>

        {@render bpk1BlockList(letter.blocks)}
    </div>

    <div class="card">
        <div class="card-header">Thumbnails</div>
        <div class="gallery">
            {#each letter.thumbnails as thumbnail, i}
                {@const title = `Thumbnail no. ${i + 1}`}
                <div>
                    <p>
                        <b>{title}</b>
                    </p>
                    <BlobImage class="thumbnail" src={thumbnail} alt={title} />
                </div>
            {/each}
        </div>
    </div>

    <div class="card">
        <div class="card-header">Doodles</div>
        <div class="gallery">
            {#each letter.sheets as sheet (sheet)}
                <div>
                    <Doodle
                        {sheet}
                        colors={letter.colors}
                        stationery={letter.stationery}
                    />
                </div>
            {/each}
        </div>
    </div>

    {#if letter.sender_mii}
        <div class="card">
            <div class="card-header">Sender</div>
            <div class="mii">
                <img class="mii" src={letter.sender_mii.url} alt="" />
                <div class="name">
                    {letter.sender_mii.name}
                    {#if letter.sender_mii.author_name}
                        (mii by: {letter.sender_mii.author_name})
                    {/if}
                </div>
            </div>
        </div>
    {/if}

    {#if letter.stationery}
        <div class="card">
            <div class="card-header">Stationery</div>
            <p>Name: {letter.stationery.name}</p>

            <div class="gallery">
                {#each [letter.stationery.background_2d, letter.stationery.background_3d, letter.stationery.mask] as stationery}
                    <BlobImage
                        src={stationery}
                        style="background-color: white;"
                        alt={"Stationery"}
                    />
                {/each}
            </div>

            <br />

            {@render bpk1BlockList(letter.stationery.blocks)}
        </div>
    {/if}
</div>

<style>
    .file {
        padding: 1em;
    }

    .card-header {
        background-color: white;
        margin: -1em;
        margin-bottom: 1em;
        padding: 0.5em 1em;
    }

    .card {
        background-color: rgba(255, 255, 255, 0.4);
        box-shadow:
            0 3px 6px rgba(0, 0, 0, 0.16),
            0 3px 6px rgba(0, 0, 0, 0.23);
        padding: 1em;
        margin-bottom: 1em;
        font-size: 18px;
    }

    .sections {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5em;
    }

    .gallery {
        display: flex;
        gap: 10px;
        flex-wrap: wrap;
    }

    .gallery :global(img.thumbnail) {
        height: 128px;
        width: 128px;
    }

    .mii {
        display: inline-flex;
        flex-direction: column;
        align-items: center;
    }

    .mii .name {
        font-size: 120%;
    }

    .mii img {
        height: 96px;
        width: 96px;
        background-color: white;
        border-radius: 50%;
    }

    button,
    .btn {
        padding: 0.5em 1em;
        background-color: white;
        border: none;
        box-shadow:
            0 3px 6px rgba(0, 0, 0, 0.16),
            0 3px 6px rgba(0, 0, 0, 0.23);
        cursor: pointer;
        font-size: 18px;
    }

    .btn.noclick {
        cursor: unset;
    }

    .header {
        display: flex;
        align-items: center;
        margin-bottom: 1em;
        justify-content: space-between;
    }

    .header .title {
        font-size: 32px;
        font-weight: bold;
    }
</style>
