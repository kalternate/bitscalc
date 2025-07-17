<script>
import H1 from "./H1.svelte";
import H2 from "./H2.svelte";
import H3 from "./H3.svelte";
import Example from "./Example.svelte";
</script>

<H1>
    Bitscalc User Manual
</H1>
<div class="font-mono flex flex-col md:flex-row my-4 gap-4 text-sm">
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

        <Example>
            5 + 3 = 8, -2 + 7 = 5
        </Example>

        <H3>Subtraction (a - b)</H3>
        <div>
            Subtracts the second integer from the first. Like addition, subtraction supports negative numbers and wraps
            around on overflow.
        </div>
        <Example>
            10 - 4 = 6, 5 - 8 = -3
        </Example>

        <H3>Multiplication (a * b)</H3>
        <div>
            Multiplies two integer values. Results are truncated to fit within the integer size.
        </div>
        <Example>
            3 * 4 = 12, -2 * 6 = -12
        </Example>

        <H3>Division (a / b)</H3>
        <div>
            Divides the first integer by the second. Truncates toward zero. Division by zero results in an error.
        </div>
        <Example>
            10 / 3 = 3, -10 / 3 = -3
        </Example>

        <H3>Remainder (a % b)</H3>
        <div>
            Returns the remainder of integer division. Follows the sign of the dividend (first operand). This operation is sometimes referred to as modulo, but remainder is more accurate in this context, as modulo typically returns only non-negative results.
        </div>
        <Example>
            10 % 3 = 1, -10 % 3 = -1
        </Example>

        <H3>Negate (-a)</H3>
        <div>
            Negates the value of an integer.
        </div>
        <Example>
            -7 = -7, -(-7) = 7
        </Example>

        <H2>Bitwise Operations</H2>
        <div>
            Bitwise operations manipulate the individual bits of integer values. Each operand is treated as a binary sequence, and the operation is applied to each pair of corresponding bits.
        </div>

        <H3>Bitwise NOT (~a)</H3>
        <div>
            Inverts each bit of the operand.
        </div>
        <Example>
            ~0 = -1, ~5 = -6
        </Example>

        <H3>Bitwise AND (a & b)</H3>
        <div>
            Performs a bitwise AND between two integers. Each bit in the result is 1 if the corresponding bits in both operands are 1.
        </div>
        <Example>
            5 & 3 = 1 (0b0101 & 0b0011 = 0b0001)
        </Example>

        <H3>Bitwise OR (a | b)</H3>
        <div>
            Performs a bitwise OR between two integers. Each bit in the result is 1 if any corresponding bit is 1.
        </div>
        <Example>
            5 | 3 = 7 (0b0101 | 0b0011 = 0b0111)
        </Example>

        <H3>Bitwise XOR (a ^ b)</H3>
        <div>
            Performs a bitwise exclusive OR. Each bit in the result is 1 only if the corresponding bits differ.
        </div>
        <Example>
            5 ^ 3 = 6 (0b0101 ^ 0b0011 = 0b0110)
        </Example>

        <H3>Left Shift (a &lt;&lt; b)</H3>
        <div>
            Shifts the bits of the first operand to the left by the number of positions specified by the second. Zeros are shifted in from the right.  This operation is the same for both signed and unsigned integers.
        </div>
        <Example>
            2 &lt;&lt; 2 = 8 (0b0010 &lt;&lt; 2 = 0b1000)
        </Example>

        <H3>Right Shift (a >> b)</H3>
        <div>
            Shifts the bits of the first operand to the right by the number of positions specified by the second. In signed mode, the leftmost bit is replicated to preserve the number's sign (arithmetic shift). In unsigned mode, zeros are shifted in from the left (logical shift).
        </div>
        <Example>
            16 >> 2 = 4 (0b00010000 >> 2 = 0b00000100)
        </Example>

        <H2>Comparison Operations</H2>
        <div>
            These operations compare two integers and return 1 for true or 0 for false.
        </div>

        <H3>Less Than (a &lt; b)</H3>
        <div>
            Returns 1 if the left operand is less than the right operand.
        </div>
        <Example>
            3 &lt; 5 = 1, 5 &lt; 3 = 0
        </Example>

        <H3>Greater Than (a > b)</H3>
        <div>
            Returns 1 if the left operand is greater than the right operand.
        </div>
        <Example>
            7 > 2 = 1, 2 > 7 = 0
        </Example>

        <H3>Less Than or Equal (a &lt;= b)</H3>
        <div>
            Returns 1 if the left operand is less than or equal to the right operand.
        </div>
        <Example>
            4 &lt;= 4 = 1, 2 &lt;= 3 = 1
        </Example>

        <H3>Greater Than or Equal (a >= b)</H3>
        <div>
            Returns 1 if the left operand is greater than or equal to the right operand.
        </div>
        <Example>
            6 >= 6 = 1, 9 >= 4 = 1
        </Example>

        <H3>Equals (a == b)</H3>
        <div>
            Returns 1 if both operands are equal.
        </div>
        <Example>
            3 == 3 = 1, 3 == 4 = 0
        </Example>

        <H3>Not Equals (a != b)</H3>
        <div>
            Returns 1 if the operands are not equal.
        </div>
        <Example>
            For example: 2 != 3 = 1, 4 != 4 = 0
        </Example>

        <H2>Logical Operations</H2>
        <div>
            Logical operations treat any non-zero value as true and 0 as false. The result is always either 1 (true) or 0 (false).
        </div>

        <H3>Logical NOT (!a)</H3>
        <div>
            Returns 1 if the operand is zero, and 0 otherwise.
        </div>
        <Example>
            !0 = 1, !5 = 0
        </Example>

        <H3>Logical AND (a && b)</H3>
        <div>
            Returns 1 if both operands are non-zero.
        </div>
        <Example>
            3 && 4 = 1, 0 && 1 = 0
        </Example>

        <H3>Logical AND (a && b)</H3>
        <div>
            Returns 1 if both operands are non-zero.
        </div>
        <Example>
            3 && 4 = 1, 0 && 1 = 0
        </Example>

        <H3>Logical OR (a || b)</H3>
        <div>
            Returns 1 if either operand is non-zero.
        </div>
        <Example>
            0 || 5 = 1, 5 || 5 = 1, 0 || 0 = 0
        </Example>

        <H3>Logical XOR (a ^^ b)</H3>
        <div>
            Returns 1 if exactly one operand is non-zero. This operator is not part of standard C, but is included in Bitscalc for symmetry.
        </div>
        <Example>
            0 ^^ 5 = 1, 5 ^^ 5 = 0, 0 ^^ 0 = 0
        </Example>

    </div>

    <div class="flex flex-col">
    <H3>Operators grouped by precedence.</H3>

    <table class="b border font-mono mr-4 min-w-72 h-fit">
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
</div>
