<script>
    import { fade } from 'svelte/transition';
    import StepDisplay from './StepDisplay.svelte';
    import TokenDisplay from './TokenDisplay.svelte';
    import { writable } from 'svelte/store';
    import FormatButton from './FormatButton.svelte';

    export let evaluation;
    export let commandChannel;
    
    let highlight = writable(0);
    let formatChannel = writable("dec");
    let format = "dec";
    
</script>

<div>

    <div class="font-mono overflow-hidden">
        {#each evaluation.command as token}
            <TokenDisplay token={token} highlight={highlight} canHighlight={evaluation.steps.length} commandChannel={commandChannel}/>
        {/each}
    </div>

    {#if evaluation.steps.length}
        <hr class="my-2">
        <div class="flex flex-col">
            <div class="flex flex-row  items-center space-x-1">
                <div class="montserrat text-xs text-zinc-400 align-middle ">FORMAT</div>
                <FormatButton bind:format={format} myFormat="dec">DEC</FormatButton>
                <FormatButton bind:format={format} myFormat="hex">HEX</FormatButton>
                <FormatButton bind:format={format} myFormat="bin">BIN</FormatButton>
            </div>
            <hr class="my-2">
            <div class="flex mx-5">
                <div class="flex-shrink">
                    {#each evaluation.steps as step}
                    <div class="flex justify-end flex-grow">
                        <StepDisplay step={step} highlight={highlight}  commandChannel={commandChannel} format={format}/>
                    </div>
                    {/each}
                </div>
            </div>

        </div>
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