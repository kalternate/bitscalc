<script>
    import { fade } from "svelte/transition";
    import logo from '../assets/logo.svg'
    import ResultDisplay from "./ResultDisplay.svelte";

    export let panelData;

</script>

<div in:fade={{ delay: 250, duration: 250 }} class="bg-zinc-800 rounded-xl p-2 text-md mt-4 justify-items-center shadow-md">
    {#if panelData.flavor === "evaluation"}
        <ResultDisplay evaluation={panelData.evaluation}/>
    {:else if panelData.flavor === "info"}
        <img alt="Bitscalc Logo" src={logo}>
        <hr class="my-2">
        <div class="font-mono">
            Bitscalc is a binary integer calculator for quickly evaluating programming expressions. It supports arithmetic, logical, and bitwise operators with C-like precedence. Step-by-step results are shown in decimal, hexadecimal, and binary. Enter an expression above to get started.
        </div>
        <div class="font-mono mt-2 text-zinc-400">
            v1.1 —
            <a href="https://github.com/kalternate/bitscalc" class="underline hover:text-sky-300">
                source
            </a>
        </div>
    {/if}
</div>