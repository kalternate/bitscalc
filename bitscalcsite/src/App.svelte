<script>
    import { fade } from 'svelte/transition';
    import { flip } from 'svelte/animate';
    import init, {evaluate_command} from 'bitscalclib';
    import ValueDisplay from './lib/ValueDisplay.svelte'
    import '@fontsource-variable/montserrat';
    
    init();

    let command = '';
    let results = []
    let counter = 0;
    let error = '';


    function evaluateInput() {
        let evaluation = evaluate_command(command);

        if (evaluation.result) {
            let result = {
                command: command,
                output: evaluation.result,
                index: counter
            }

            counter += 1

            results.push(result);
            command = '';
            error = '';
            results = results;
        } else if (evaluation.error) {
            error = evaluation.error;
        }
    }

    function handleKeydown(event) {
        if (event.key === "Enter") {
            evaluateInput();
        }
    }

</script>

<main class="flex flex-grow flex-col">
    <div class=" flex flex-grow justify-center">
        <input class="text-md bg-zinc-800 border-zinc-700 rounded-xl p-2 font-mono border-2 focus:outline-none focus:ring-0  focus:border-indigo-500 transition-colors placeholder:text-zinc-500 flex-grow max-w-[64rem] shadow-md" placeholder="enter an expression..." bind:value={command} on:keydown={handleKeydown}/>
    </div>

    <div class="flex justify-center">
        <div class="flex flex-grow flex-col-reverse justify-center justify-items-center max-w-[64rem]">
            {#each results as result (result.index)}
            <div animate:flip={{ delay: 0, duration: 250}} in:fade={{ delay: 250, duration: 250 }} class="bg-zinc-800 flex-shrink rounded-xl p-2 text-md mt-4  justify-items-center shadow-md">
                <div class="font-mono">{result.command}</div>
                <hr class="my-2">
                <ValueDisplay value={result.output}/>
            </div>
            {/each}

            <!--{#if error}
                <div class="bg-red-300 text-red-900 rounded-xl px-2 py-1 my-1 border-red-400 border-2" in:fade={{ delay: 0, duration: 250 }}>{error}</div>
            {/if}-->

            {#if error}
                <div class="text-red-400 mx-1" in:fade={{ delay: 0, duration: 250 }}>{error}</div>
            {/if}
        </div>
    </div>
    
</main>

<style>

</style>
