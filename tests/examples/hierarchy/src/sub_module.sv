// Sub-module that is instantiated by hierarchy_top
module sub_module (
    input wire clk,
    input wire rst_n,
    input wire [7:0] data_in,
    output reg [7:0] data_out
);

    // Sequential logic
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            data_out <= 8'h00;
        end else begin
            data_out <= data_in;
        end
    end

endmodule
