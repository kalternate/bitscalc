<script>
    import { fade } from "svelte/transition";
    import ResultDisplay from "./ResultDisplay.svelte";
    import LandingDisplay from "./LandingDisplay.svelte";
    import HelpDisplay from "./HelpDisplay.svelte";

    export let panelData;
    export let commandChannel;

</script>

<div in:fade={{ delay: 250, duration: 250 }} class="bg-zinc-800 rounded-xl p-2 text-md mt-4 shadow-md">
    {#if panelData.flavor === "evaluation"}
        <ResultDisplay evaluation={panelData.evaluation}/>
    {:else if panelData.flavor === "info"}
        <LandingDisplay commandChannel={commandChannel}/>
    {:else if panelData.flavor === "help"}
        <HelpDisplay/>
    {/if}
</div>