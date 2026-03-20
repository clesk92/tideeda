module counter #(
    parameter WIDTH = 8
) (
    input  wire clk,
    input  wire rst_n,
    input  wire enable,
    output reg  [WIDTH-1:0] count
);

    // 线网定义
    wire [WIDTH-1:0] next_count;
    wire               carry;

    // 连续赋值 - 组合逻辑
    assign next_count = count + 1'b1;
    assign carry = &count;  // 当所有位都为1时，carry为1

    // 过程块 - 时序逻辑
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            count <= 0;
        end else if (enable) begin
            count <= next_count;
        end
    end

endmodule
