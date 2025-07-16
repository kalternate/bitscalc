<script>
import H1 from "./H1.svelte";
import H2 from "./H2.svelte";
import H3 from "./H3.svelte";
import Example from "./Example.svelte";
</script>

<H1>
    Bitscalc User Manual
</H1>
<div class="font-mono flex flex-row my-4 gap-4 text-sm">
    <div class="flex flex-col">
        <div class="font-mono">
            Enter an expression above to get started. Operations are listed in order of precedence. Only integer values are
            supported. Operations follow C-like behavior. Division results are truncated toward zero. Bitwise operations
            perform logical operations on corresponding bits of integers. Logical operations treat 0 as false and any
            non-zero value as true. Boolean operations return 1 for true and 0 for false.
        </div>

        <H2>
            Arithmetic Operations
        </H2>
        <div>
            Arithmetic operations include basic mathematical operations like addition, subtraction,
            multiplication, division, and remainder. Division results are truncated toward zero. The unary
            minus operator negates a value. Operations follow standard mathematical precedence rules where
            multiplication, division, and remainder are performed before addition and subtraction.
        </div>

        <H3>Addition (a + b)</H3>
        <div>
            Adds two integer values together. Results that exceed the maximum value will wrap around to the minimum
            value. Negative values are supported when in signed mode.
        </div>

        <div class="mt-2 text-zinc-400">For example:</div>
        <div class="flex flex-row gap-2">
        <Example>
            5 + 3 = 8
        </Example>
        <Example>
            -2 + 7 = 5
        </Example>
        </div>

        <H3>Subtraction (a - b)</H3>
        <div>
            Subtracts the second integer from the first. Like addition, subtraction supports negative numbers and wraps
            around on overflow.
        </div>

        <div class="mt-2 text-zinc-400">For example:</div>
        <div class="flex flex-row gap-2">
            <Example>
                10 - 4 = 6
            </Example>
            <Example>
                5 - 8 = -3
            </Example>
        </div>

        <H3>Multiplication (a * b)</H3>
        <div>
            Multiplies two integer values. Results are truncated to fit within the integer size.
            For example: 3 * 4 = 12, -2 * 6 = -12
        </div>

        <H3>Division (a / b)</H3>
        <div>
            Divides the first integer by the second. Truncates toward zero. Division by zero results in an error.
            For example: 10 / 3 = 3, -10 / 3 = -3
        </div>

        <H3>Remainder (a % b)</H3>
        <div>
            Returns the remainder of integer division. Follows the sign of the dividend (first operand). This operation is sometimes referred to as modulo, but remainder is more accurate in this context, as modulo typically returns only non-negative results.
            For example: 10 % 3 = 1, -10 % 3 = -1
        </div>

        <H3>Negate (-a)</H3>
        <div>
            Negates the value of an integer.
            For example: -7 = -7, -(-7) = 7
        </div>

    <H2>Bitwise Operations</H2>
    <div>
        Bitwise operations manipulate the individual bits of integer values. Each operand is treated as a binary sequence, and the operation is applied to each pair of corresponding bits.
    </div>

    <H3>Bitwise NOT (~a)</H3>
    <div>
        Inverts each bit of the operand.
        For example: ~0 = -1, ~5 = -6
    </div>
    </div>

    <table class="b border font-mono mr-4 min-w-72">
        <tr>
            <th class="border py-1 px-2 text-right">-<br> !<br> ~</th>
            <th class="border p-1 text-left">Negate<br> Logical not<br> Bitwise not</th>
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
</div>