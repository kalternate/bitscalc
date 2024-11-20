<script>
    import { fade } from 'svelte/transition';
    import init, {evaluatetojson} from '../../lib/pkg';
    import '@fontsource-variable/montserrat';
    import { flip } from 'svelte/animate';
    import { writable } from 'svelte/store';    
    import Panel from './lib/Panel.svelte';
    import SettingButton from './lib/SettingButton.svelte';
    init();

    let userInput = '';
    let panels = []
    let counter = 0;
    let error = '';

    let signed = true;
    let size = 64;

    let commandChannel = writable("");

    commandChannel.subscribe(evaluateCommand);
    evaluateCommand("info")

    function evaluateCommand(command) {
        if (command === "") {
            return;
        } else if (command === "info" || command === "help") {
            let panelData = {
                flavor: command,
                index: counter
            }
            panels.push(panelData);
        } else {
            window.scrollTo({ top: 0, behavior: 'smooth' });

            let evalJson = evaluatetojson(command, signed, BigInt(size));
            let evaluation = JSON.parse(evalJson);

            if (evaluation.token) {
                let panelData = {
                    evaluation: evaluation,
                    flavor: "evaluation",
                    index: counter
                }
                panels.push(panelData);
            } else if (evaluation.error) {
                error = evaluation.error;
                return;
            }
        }

        counter += 1
        userInput = '';
        error = '';
        panels = panels;
        commandChannel.set("")
    }

    function handleKeydown(event) {
        if (event.key === "Enter") {
            commandChannel.set(userInput)
        }
    }

    let signedChannel = writable(true);
    let sizeChannel = writable(64);

    signedChannel.subscribe(v => signed = v);
    sizeChannel.subscribe(v => size = v);

</script>

<main class="flex flex-grow flex-col">
    <div class=" flex flex-grow justify-center">
        <input class="text-md bg-zinc-800 border-zinc-700 rounded-xl p-2 font-mono border-2 focus:outline-none focus:ring-0  focus:border-green-300 transition-colors placeholder:text-zinc-500 flex-grow max-w-[64rem] shadow-md" placeholder="enter an expression..." bind:value={userInput} on:keydown={handleKeydown}/>
    </div>

    <div class="flex justify-center">
        <div class="flex flex-grow flex-col justify-center justify-items-center max-w-[64rem]">
            <div class="flex flex-row gap-2 mt-1">
                <SettingButton buttonChannel={signedChannel} value={true}>signed</SettingButton>
                <SettingButton buttonChannel={signedChannel} value={false}>unsigned</SettingButton>
                <div>
                —
                </div>
                <SettingButton buttonChannel={sizeChannel} value={64}>64-bit</SettingButton>
                <SettingButton buttonChannel={sizeChannel} value={32}>32-bit</SettingButton>
                <SettingButton buttonChannel={sizeChannel} value={16}>16-bit</SettingButton>
                <SettingButton buttonChannel={sizeChannel} value={8}>8-bit</SettingButton>
            </div>

            {#if error}
                <div class="text-red-400 mx-1" in:fade={{ delay: 0, duration: 250 }}>{error}</div>
            {/if}

            <div class="flex flex-grow flex-col-reverse">
                {#each panels as panelData (panelData.index)}
                <div animate:flip={{ delay: 0, duration: 250}}>
                    <Panel panelData={panelData} commandChannel={commandChannel}/>
                </div>
                {/each}
            </div>
        </div>
    </div>
    
</main>

<style>

</style>
