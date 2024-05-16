<script>
    import { fade } from 'svelte/transition';
    import init, {evaluatetojson} from 'bitscalclib';
    import '@fontsource-variable/montserrat';
    import { flip } from 'svelte/animate';
    import ResultDisplay from './lib/ResultDisplay.svelte';
    import { writable } from 'svelte/store';    
    import logo from './assets/logo.svg'
    init();

    let userInput = '';
    let results = []
    let counter = 0;
    let error = '';

    let commandChannel = writable("");

    commandChannel.subscribe(evaluateCommand);

    function evaluateCommand(command) {
        if (command === "") {
            return;
        }

        window.scrollTo({ top: 0, behavior: 'smooth' });

        let evalJson = evaluatetojson(command);
        let evaluation = JSON.parse(evalJson);
        
        if (evaluation.token) {
            let result = {
                evaluation: evaluation,
                index: counter
            }

            counter += 1

            results.push(result);
            userInput = '';
            error = '';
            results = results;
            commandChannel.set("")
        } else if (evaluation.error) {
            error = evaluation.error;
        }
    }

    function handleKeydown(event) {
        if (event.key === "Enter") {
            commandChannel.set(userInput)
        }
    }

</script>

<main class="flex flex-grow flex-col">
    <div class=" flex flex-grow justify-center">
        <input class="text-md bg-zinc-800 border-zinc-700 rounded-xl p-2 font-mono border-2 focus:outline-none focus:ring-0  focus:border-sky-300 transition-colors placeholder:text-zinc-500 flex-grow max-w-[64rem] shadow-md" placeholder="enter an expression..." bind:value={userInput} on:keydown={handleKeydown}/>
    </div>

    <div class="flex justify-center">
        <div class="flex flex-grow flex-col-reverse justify-center justify-items-center max-w-[64rem]">

            <div class="bg-zinc-800 rounded-xl p-2 text-md mt-4  justify-items-center shadow-md">
                <img alt="Bitscalc" src={logo}>
                <hr class="my-2">
                <div class="font-mono">
                    This calculator evaluates binary integer operations including arithmetic, bitwise, and logical operators. It uses C-style operator precedence and will evaluate sub-expressions in parentheses first. Use it to reason about how integer operators are evaluated in programming languages like C. Results will be displayed in decimal, hexadecimal, and binary.
                </div>


            </div>

            {#each results as result (result.index)}
                <div animate:flip={{ delay: 0, duration: 250}}>
                    <ResultDisplay evaluation={result.evaluation} commandChannel={commandChannel}/>
                </div>
            {/each}

            {#if error}
                <div class="text-red-400 mx-1" in:fade={{ delay: 0, duration: 250 }}>{error}</div>
            {/if}
        </div>
    </div>
    
</main>

<style>

</style>
