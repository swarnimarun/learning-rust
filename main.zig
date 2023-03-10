const std = @import("std");

const Str = struct {
    // null terminated string slice
    data: [:0]const u8,
    pub fn get(self: Str, idx: usize) ?u8 {
        if (self.data.len > idx) {
            return self.data[idx];
        } else {
            return null; // equivalent to None
        }
    }
    pub fn split_at(self: Str, idx: usize) ?struct { []const u8, []const u8 } {
        if (self.data.len > idx) {
            return .{ self.data[0..idx], self.data[idx..] };
        } else {
            return null; // equivalent to None
        }
    }
};

pub fn main() !void {
    var src = Str{ .data = "this is a string" };
    if (src.split_at(5)) |t| {
        var x = t[0];
        var y = t[1];
        std.debug.print("{s}\n{s}", .{ x, y });
    }
    return;
}
