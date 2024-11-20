<script>
    import logo from '../assets/logo.svg'
    export let commandChannel;

</script>

<img alt="Bitscalc Logo" src={logo}>
<hr class="my-2">
<div class="font-mono">
                Bitscalc is a binary integer calculator for quickly evaluating programming expressions. It supports arithmetic, logical, and bitwise operators with C-like precedence. Step-by-step results are shown in decimal, hexadecimal, and binary. Enter an expression above to get started.
</div>
<div class="font-mono italic mt-2 text-zinc-400">
    If you use Bitscalc, I'd love to hear from you! Submit feedback using the link below or email me at keaganmedwards@gmail.com.
</div>
<div class="font-mono text-zinc-400">
    v1.2 
    —
    <button on:click={commandChannel.set("help")} class="underline text-sky-300">
        help
    </button> 
    <a href="https://github.com/kalternate/bitscalc" class="underline text-sky-300">
        source
    </a>
    <a href="https://forms.gle/YdA3xi19RzMr6SFG7" class="underline text-sky-300">
        feedback
    </a>
</div>

<div class="font-mono mt-4 text-zinc-400">
    <h2 class="text-xl italic ">Changelog</h2>
    <hr class="border-zinc-400">
    <div class="mt-4">Version v1.2 — 11-20-2024</div>
    <div>— Revamped site style</div>
    <div>— New favicon</div>
    <div>— Added feeback form </div>
    <div>— Added changelog </div>

    <div class="mt-4">Version v1.1 — 6-15-2024</div>
    <div>— Added support for multiple integer formats</div>
    <div>— Added help page </div>
    <div>— Improved binary formating </div>
    <div>— Style & bug fixes </div>

    <div class="mt-4">Version v1.0 — 5-19-2024</div>
    <div>— Initial release</div>
</div>
