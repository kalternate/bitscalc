<script>
    import { fade } from 'svelte/transition';
    import StepDisplay from './StepDisplay.svelte';
    import TokenDisplay from './TokenDisplay.svelte';
    import { writable } from 'svelte/store';

    export let evaluation;
    export let commandChannel;

    let highlight = writable(0);
    
</script>

<div in:fade={{ delay: 250, duration: 250 }} class="bg-zinc-800 flex-shrink rounded-xl p-2 text-md mt-4  justify-items-center shadow-md">

    <div class="font-mono">
        {#each evaluation.command as token}
            <TokenDisplay token={token} highlight={highlight} canHighlight={evaluation.steps.length} commandChannel={commandChannel}/>
        {/each}
    </div>

    {#if evaluation.steps.length}
        <hr class="my-2">
        {#each evaluation.steps as step}
            <div class="ml-5 flex">
                <div class="">
                    <StepDisplay step={step} highlight={highlight}  commandChannel={commandChannel}/>
                </div>
            </div>
        {/each}
    {/if}

    <hr class="my-2">
    <div class="flex flex-col">
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">DEC</div>
            <TokenDisplay token={evaluation.token} highlight={highlight} format="dec" canHighlight={false}/>
        </div>
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">HEX</div>
            <TokenDisplay token={evaluation.token} highlight={highlight} format="hex" canHighlight={false}/>
        </div>
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">BIN</div>
            <TokenDisplay token={evaluation.token} highlight={highlight} format="bin" canHighlight={false}/>
        </div>
    </div>
    
</div>