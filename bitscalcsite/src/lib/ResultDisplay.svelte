<script>
    import { fade } from 'svelte/transition';
    import ValueDisplay from './ValueDisplay.svelte'
    import StepDisplay from './StepDisplay.svelte';

    export let evaluation;
    
</script>

<div in:fade={{ delay: 250, duration: 250 }} class="bg-zinc-800 flex-shrink rounded-xl p-2 text-md mt-4  justify-items-center shadow-md">
    <div class="font-mono">{evaluation.command}</div>
    <hr class="my-2">
    {#each evaluation.steps as step}
        <div class="ml-5 flex">
            <div class="">
                <StepDisplay step={step}/>
            </div>
        </div>
    {/each}
    <hr class="my-2">
    <div class="flex flex-col">
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">DEC</div>
            <div class="font-mono">{evaluation.format.dec}</div>
        </div>
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">HEX</div>
            <div class="font-mono">{evaluation.format.hex}</div>
        </div>
        <div class="flex flex-row space-x-2 items-center">
            <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">BIN</div>
            <div class="font-mono">{evaluation.format.bin}</div>
        </div>
    </div>
</div>