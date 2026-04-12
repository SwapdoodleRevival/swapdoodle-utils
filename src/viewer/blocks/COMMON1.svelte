<script lang="ts">
    import Card from "../../components/Card.svelte";
    import type {
        BasicDateTime,
        BPK1Block,
        BPK1File,
        CommonInfo,
    } from "../../lib/libdoodle/libdoodle.svelte";

    let {
        file,
        block,
    }: {
        file: BPK1File;
        block: BPK1Block;
    } = $props();

    let common_info: CommonInfo = $derived(block.parse_commoninfo());

    function basic_date_time_to_date(date: BasicDateTime): string {
        const datePart = `${(2000 + date.year).toString().padStart(4, "0")}-${date.month.toString().padStart(2, "0")}-${date.day.toString().padStart(2, "0")}`;
        const timePart = `${date.hour.toString().padStart(2, "0")}:${date.minute.toString().padStart(2, "0")}:${date.second.toString().padStart(2, "0")}`;
        return `${datePart} ${timePart}`;
    }
</script>

<Card style="info" title="About COMMON1" class="mb-2">
    COMMON1 blocks contain general info about a Swapdoodle letter.
</Card>

<div>
    <div>
        <span class="font-bold">Note ID</span>: {common_info.note_id}
    </div>
    <div>
        <span class="font-bold">Reply to note ID</span>: {common_info.reply_to_note_id}
    </div>
    <div>
        <span class="font-bold">Sender PID</span>: {common_info.sender_pid}
    </div>
    <div>
        <span class="font-bold">Date</span>: {basic_date_time_to_date(
            common_info.sent,
        )}
    </div>
</div>
