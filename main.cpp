#include <iostream>
#include <vector>
#include <optional>
#include <algorithm>
#include <span>
#include <format>

using namespace std;

typedef char u8;
typedef size_t usize;
struct Str {
    span<u8> data;
    auto get(usize idx) -> optional<u8> {
        if (idx < this->data.size()) {
            return {this->data[idx]};
        } else {
            return {};
        }
    }
    auto split_at(usize idx) -> optional<tuple<span<u8>, span<u8>>> {
        if (idx < this->data.size()) {
            return {{
                this->data.subspan(0, idx),
                this->data.subspan(idx),           
            }};
        } else {
            return {};
        }
    }
};

int main() {
    string src = "this is a string";
    Str s{{src.begin(), src.end()}};
    auto res = s.split_at(5);
    if (res.has_value()) {
        auto [x, y] = res.value();
        cout << format("{}\n{}\n", string{x.begin(), x.end()}, string{y.begin(), y.end()});
    }
}
