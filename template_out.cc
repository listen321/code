#include <ostream>
#include <type_traits>
#include <utility>

template<bool condition, typename Body>
struct WhileLoop;

template<typename Body>
struct WhileLoop<true, Body>
{
    using type = typename WhileLoop<Body::cond_value, typename Body::next_type>::type;
};

template<typename Body>
struct WhileLoop<false, Body>
{
    using type = typename Body::res_type;
};

template<typename Body>
struct While
{
    using type = typename WhileLoop<Body::cond_value, Body>::type;
};

template<typename T, T v>
struct integral_constant
{
    static const T value = v;
    using value_type = T;
    using type = integral_constant;
};

template<int result, int n>
struct SumLoop
{
    static const bool cond_value = n != 0;
    static const int res_value = result;
    using res_type = integral_constant<int, res_value>;
    using next_type = SumLoop<result + n, n - 1>;
};

template<int n>
struct Sum
{
    using type = SumLoop<0, n>;
};

#include <vector>
template<
    template <typename, typename>
    class OutContainer = std::vector,
    typename F, class R>
auto fmap(F&& f, R&& inputs)
{
    using result_type = std::decay_t<decltype(f(*inputs.begin()))>;

    OutContainer<result_type, std::allocator<result_type>> result;
    for(auto&& item : inputs)
    {
        result.push_back(f(item));
    }

    return result;
}

template<typename T>
struct has_reserve
{
    struct good { char dummy; };
    struct bad { char dummy[2]; };

    template<class U, void (U::*)(size_t)>
    struct SFINAE{};

    template<class U>
    static good reserve(SFINAE<U, &U::reserve>);

    template<class U>
    static bad reserve(...);

    static const bool value = sizeof(reserve<T>(nullptr)) == sizeof(good);
};

int add_1(int x)
{
    return x+1;
}

template<typename T>
struct is_pair : std::false_type
{};

template<typename T, typename U>
struct is_pair<std::pair<T, U>> : std::true_type
{};

template<typename T>
inline constexpr bool is_pair_v = is_pair<T>::value;

template<typename T>
struct has_output_function
{
    template<class U>
    static auto output(U* ptr)-> decltype(std::declval<std::ostream&>() << *ptr, std::true_type());

    template<class U>
    static std::false_type output(...);

    static constexpr bool value = decltype(output<T>(nullptr))::value;
};

template<typename T>
inline constexpr bool has_output_function_v = has_output_function<T>::value;

template<typename T, typename U>
std::ostream& operator<<(std::ostream& os, const std::pair<T, U>& pr);

template<typename T, typename Cont>
auto output_element(std::ostream& os, const T& element, const Cont&, const std::true_type)
    -> decltype(std::declval<typename Cont::key_type>(), os);

template<typename T, typename Cont>
auto output_element(std::ostream& os, const T& element, const Cont&, ...) -> decltype(os);

template<
    typename T,
    typename = std::enable_if_t<!has_output_function_v<T>>>
auto operator<<(std::ostream& os, const T& container) -> decltype(container.begin(), container.end(), os)
{
    using std::decay_t;
    using std::is_same_v;

    using element_type = decay_t<decltype(*container.begin())>;
    constexpr bool is_char_v = is_same_v<element_type, char>;
    if constexpr (!is_char_v)
    {
        os << "{";
    }
    
    if(!container.empty())
    {
        auto end = container.end();
        bool on_first_element = true;
        for(auto it = container.begin(); it != end; ++it)
        {
            if constexpr(is_char_v)
            {
                if(*it == '\0')
                    break;
            }
            else
            {
                if (!on_first_element)
                {
                    os << ", ";
                }
                else
                {
                    on_first_element = false;
                }
            }
            output_element(os, *it, is_pair<element_type>());
            
        }

    }

    if constexpr (!is_char_v)
    {
        os << "}";
    }

    return os;
}

template<typename T, typename Cont>
auto output_element(std::ostream& os, const T& element, const Cont&, const std::true_type)
    -> decltype(std::declval<typename Cont::key_type>(), os)
{
    os << element.first << " => "
        << element.second;
    return os;
}

template<typename T, typename Cont>
auto output_element(std::ostream& os, const T& element, const Cont&, ...)
    -> decltype(os)
{
    os << element;
    return os;
}

template<typename T, typename U>
std::ostream& operator<<(std::ostream& os, const std::pair<T, U>& pr)
{
    os << '(' << pr.first << ", "
        << pr.second << ')';
    return os;
}

#include <map>
#include <iostream>
int main()
{
    // auto t = While<Sum<10>::type>::type::value;
    // std::cout << t << std::endl;
    // std::vector<int> v{1, 2, 3, 4, 5};

    // auto result = fmap(add_1, v);
    // for(auto i : result)
    // {
    //     std::cout << i << std::endl;
    // }
    std::vector<int> arr {1, 2, 3, 4, 7, 10};
    std::cout << arr << std::endl;;

    std::vector<std::pair<int, std::string>> arr_p{{1, "a"}, {2, "b"}, {3, "c"}};
    std::cout << arr_p << std::endl;

    std::map<int, char> m;
    m[4] = 'd';
    m[5] = '\0';
    m[6] = 'e';
    m[7] = 'f';
    std::cout << m << std::endl;
    return 0;
}
