<script>
    import { fade } from 'svelte/transition';
    import init, {evaluatetojson} from 'bitscalclib';
    import '@fontsource-variable/montserrat';
    import { flip } from 'svelte/animate';
    import ResultDisplay from './lib/ResultDisplay.svelte';

    init();

    let command = '';
    let results = []
    let counter = 0;
    let error = '';


    function evaluateInput() {
        let evaluation = JSON.parse(evaluatetojson(command));

        if (evaluation.format) {
            let result = {
                evaluation: evaluation,
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
                <div animate:flip={{ delay: 0, duration: 250}}>
                    <ResultDisplay evaluation={result.evaluation}></ResultDisplay>
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
