use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (history, set_history) = create_signal(Vec::<i32>::new());

    let increment = move |_: leptos::ev::MouseEvent| {
        let current = count.get();

        set_history.update(|hist| {
            hist.push(current);
        });

        set_count.set(current + 1);
    };

    let decrement = move |_: leptos::ev::MouseEvent| {
        let current = count.get();

        if current > 0 {
            set_history.update(|hist| {
                hist.pop();
            });
            set_count.set(current - 1);
        }
    };

    //o move no caso é como se fosse um ponteiro compartilhado, isso devido ao signal
    //👉 count NÃO é um valor comum O SEGREDO => SIGNAL

    let reset = move |_: leptos::ev::MouseEvent| {
        set_count.set(0);
        set_history.set(Vec::new());
    };

    view! {
            <Stylesheet href="/assets/output.css"/>

            <div class="min-h-screen relative pt-12 bg-black text-white flex flex-col items-center gap-6 font-mono">


                // Card principal
                <div class=" p-8 flex flex-col items-center gap-6">

                    <h1 class="text-xl md:text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-pink-500 via-purple-500 to-cyan-500 drop-shadow-[0_0_10px_rgba(0,255,255,0.8)]">
                    "Valor atual"
                </h1>

                    <p class="text-5xl font-bold text-cyan-400 drop-shadow-[0_0_10px_rgba(0,255,255,0.9)]">
                        {count}
                    </p>

                    // Botões
                    <div class="flex gap-4">

                        <button
                            on:click=increment
                            class="px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-500 hover:scale-110 hover:shadow-[0_0_15px_rgba(0,255,255,0.9)] transition-all duration-200"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
                        </button>

                        <button
                            on:click=decrement
                            disabled=move || count.get() == 0
                            class="px-4 py-2 rounded-xl bg-gradient-to-r from-pink-500 to-red-500 disabled:opacity-30 hover:scale-110 hover:shadow-[0_0_15px_rgba(255,0,150,0.9)] transition-all duration-200"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-5 h-5">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
    </svg>
                        </button>

                        <button
                            on:click=reset
                            disabled=move || count.get() == 0
                            class="px-4 py-2 rounded-xl bg-gradient-to-r from-purple-500 to-indigo-500 disabled:opacity-30 hover:scale-110 hover:shadow-[0_0_15px_rgba(150,0,255,0.9)] transition-all duration-200"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-5 h-5">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v6h6M20 20v-6h-6M5 19a9 9 0 0014-7M19 5a9 9 0 00-14 7" />
    </svg>
                        </button>

                    </div>
                </div>

                // Histórico
                {move || {
                    if history.get().is_empty() {
                        ().into_view()
                    } else {
                        view! {
                            <div class="absolute mt-[18rem] h-64 w-64 bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-4 shadow-[0_0_20px_rgba(255,0,255,0.2)]">

                                <h3 class="text-center text-pink-400 mb-2">
                                    "Histórico"
                                </h3>

                                <ul class="space-y-1 text-center text-sm text-gray-300 max-h-48 overflow-y-auto">
                                    {history
                                        .get()
                                        .into_iter()
                                        .rev()
                                        .map(|v| view! {
                                            <li class="hover:text-cyan-400 transition">
                                                {v}
                                            </li>
                                        })
                                        .collect_view()
                                    }
                                </ul>

                            </div>
                        }.into_view()
                    }
                }}

            </div>
        }
}
