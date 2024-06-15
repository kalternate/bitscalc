<script>
    import { fade } from "svelte/transition";
    import logo from '../assets/logo.svg'
    import ResultDisplay from "./ResultDisplay.svelte";

    export let panelData;
    export let commandChannel;

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
            v1.1 
            —
            <button on:click={commandChannel.set("help")} class="underline hover:text-sky-300">
                help
            </button> 
            <a href="https://github.com/kalternate/bitscalc" class="underline hover:text-sky-300">
                source
            </a> 
        </div>
    {:else if panelData.flavor === "help"}
        <div class="font-mono font-bold">
            Help
        </div>
        <hr class="my-2">
        <div class="font-mono flex flex-row m-4">
            <table class="b border font-mono mr-4 min-w-72">
                <tr>
                    <th class="border py-1 px-2 text-right">-<br> !<br> ~</th>
                    <th class="border p-1 text-left">Negative<br> Logical not<br> Bitwise not</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">*<br> /<br> %</th>
                    <th class="border p-1 text-left">Multiplication<br>Division<br>Remainder</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">+<br> -</th>
                    <th class="border p-1 text-left">Addition<br>Subtract</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">&lt;&lt;<br>&gt;&gt;</th>
                    <th class="border p-1 text-left">Left bitshift<br>Right bitshift</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">&lt;<br>&gt;<br>&lt;=<br>&gt;=</th>
                    <th class="border p-1 text-left">Less than<br>Greater than<br>Less than or equals<br>Greater than or equals</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">==<br>!=</th>
                    <th class="border p-1 text-left">Equals<br>Not equals</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">&</th>
                    <th class="border p-1 text-left">Bitwise and</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">^</th>
                    <th class="border p-1 text-left">Bitwise xor</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">|</th>
                    <th class="border p-1 text-left">Bitwise or</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">&&</th>
                    <th class="border p-1 text-left">Logical and</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">^^</th>
                    <th class="border p-1 text-left">Logical xor</th>
                </tr>
                <tr>
                    <th class="border py-1 px-2 text-right">||</th>
                    <th class="border p-1 text-left">Logical or</th>
                </tr>
            </table>
            <div class="font-mono">
                Enter an expression to get started. Listed operations are organized by precedence. Non-integer value are not supported. Operations largely  mimic the behavior of C. Remainders are dropped from division. Bitwise operations preform logical operations on likewise bits of the integers. Logical operations treat 0 as a false and any other number as true. Operations that result in boolean values return 1 if true and 0 if false. 
            </div>
        </div>
    {/if}
</div>