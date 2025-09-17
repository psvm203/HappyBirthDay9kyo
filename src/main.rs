use chrono::{DateTime, Utc};
use sycamore::prelude::*;

const POKEMON_RELEASE_DATE: &str = "2025-10-16T00:00:00Z";

#[component]
fn Talk() -> View {
    view! {
        div(class="flex justify-center items-center h-screen mt-10") {
            div(class="mockup-phone") {
                div(class="mockup-phone-camera") {}
                div(class="mockup-phone-display text-white bg-sky-200 grid") {
                    div(class="mx-4") {
                        div(class="chat chat-start mt-20") {
                            div(class="chat-image avatar") {
                                div(class="w-10 rounded-full") {
                                    img(src="https://avatars.githubusercontent.com/u/100560031?v=4") {}
                                }
                            }
                            div(class="chat-bubble") {
                                "교황이 형, 생일 축하해요!"
                            }
                        }
                        div(class="chat chat-end mt-10") {
                            div(class="chat-image avatar") {
                                div(class="w-10 rounded-full") {
                                    img(src="https://avatars.githubusercontent.com/u/49135176?v=4") {}
                                }
                            }
                            div(class="chat-bubble") {
                                "이... 이게 뭐고..."
                            }
                        }
                        div(class="chat chat-start mt-10") {
                            div(class="chat-image avatar") {
                                div(class="w-10 rounded-full") {
                                    img(src="https://avatars.githubusercontent.com/u/100560031?v=4") {}
                                }
                            }
                            div(class="chat-bubble") {
                                "색다른 방법으로 축하하고 싶어서 2시간동안 만들었어요😀"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Loading() -> View {
    view! {
        div(class="flex justify-center items-center mt-10") {
            div(class="mockup-code w-1/4 md:w-1/2") {
                pre(data-prefix="$") {
                    code { "gcc HappyBirthDay9kyo.c" }
                }
                pre(data-prefix="$") {
                    code { "./HappyBirthDay9kyo" }
                }
                pre(data-prefix=">", class="text-success") {
                    code { "Happy Birthday, 9kyohwang!" }
                }
            }
        }
    }
}

#[component]
fn Trade() -> View {
    //(구)교환
    view! {
        div(class="flex justify-center items-center mt-50") {
            img(src="assets/google.png") {}
        }
        div(class="text-4xl flex justify-center items-center mt-5") {
            "구교환이 구교황의 오타가 될 때까지 정진하시길..."
        }
    }
}

#[component]
fn New9Kyo() -> View {
    view! {
        div(class="flex justify-center items-center mt-50") {
            img(src="assets/new9kyo.png") {}
        }
        div(class="text-5xl flex justify-center items-center mt-1") {
            "새 교황의 반댓말은?" br{} "구교황ㅋㅋㅋㅋㅋㅋㅋ"
        }
    }
}

#[component]
fn Pokemon() -> View {
    view! {
        div(class="text-5xl flex justify-center items-center h-screen mt-1") {
            "생일 선물은? 두구두구..."
        }
        div(class="flex justify-center items-center mt-10") {
            img(src="assets/az.jpg") {}
        }
        div(class="text-5xl flex justify-center items-center mt-10") {
            "축하합니다! 교황쿤은(는) Pokémon LEGENDS Z-A을(를) 획득했다!"
        }
    }
}

fn parse_date(date_string: &'static str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(date_string)
        .expect("Invalid Date")
        .with_timezone(&Utc)
}

#[component]
fn Timer() -> View {
    let release_time = parse_date(POKEMON_RELEASE_DATE);

    let now = Utc::now();
    let duration: chrono::Duration = release_time.signed_duration_since(now);

    view! {
        div(class="text-5xl flex justify-center items-center mt-20") {
            "발매까지 " (duration.num_days()) "일 남았어요!"
        }
        div(class="text-5xl flex justify-center items-center mt-5") {
            "발매까지 " (duration.num_hours()) "시간 남았어요!"
        }
        div(class="text-5xl flex justify-center items-center mt-5") {
            "발매까지 " (duration.num_minutes()) "분 남았어요!"
        }
        div(class="text-5xl flex justify-center items-center mt-5") {
            "발매까지 " (duration.num_seconds()) "초 남았어요!"
        }
        div(class="text-5xl flex justify-center items-center mt-5") {
            "발매까지 " (duration.num_milliseconds()) "밀리초 남았어요!"
        }
        div(class="text-sm flex justify-center items-center mt-5") {
            "사용자분들의 브라우저 성능을 위하여 실시간 타이머 기능은 개발하지 않았음을 알려드립니다🙇🙇 절대 만들기 빡세서 그런 거 아님"
        }
    }
}

#[component]
fn Me2() -> View {
    view! {
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/metoo.png") {}
        }
        div(class="text-5xl flex justify-center items-center mt-5 mb-50") {
            "(아님, 8개월 남음)"
        }
    }
}

fn Why() -> View {
    view! {
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/hhh.jpg") {}
        }
        div(class="text-5xl flex justify-center items-center mt-5 mb-50") {
            "갑자기 왜 이러는 걸까요?"
        }
    }
}

fn Squirrel() -> View {
    view! {
        div(class="flex justify-center items-center mt-100") {
            video(src="assets/squirrel.mp4", controls=true) {}
        }
        div(class="text-3xl flex justify-center items-center mt-5 mb-50") {
            "2024년 5월, 대구 동성로에서 놀았던 걸 기억하시나요?"
        }
        div(class="flex justify-center items-center mt-50") {
            img(src="assets/sister.jpg", width="35%") {}
        }
        div(class="text-3xl flex justify-center items-center mt-5 mb-50") {
            "저녁을 제가 결제했는데, 누나가 밥값을 주셨어요!"
        }
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/remember.jpg", width="35%") {}
        }
        div(class="text-3xl flex justify-center items-center mt-5 mb-50") {
            "카톡에 저녁 정산을 안 올리고, 눈치챈 사람한테 인센티브를 제공하려했지만..."
        }
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/rice.jpg", width="35%") {}
        }
        div(class="text-2xl flex justify-center items-center mt-5 mb-50") {
            "교황쿠가 다른 정산이랑 착각해서 그냥 정산을 받고 다른 방법으로 돌려드리려했어요..."
        }
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/nvidia.png", width="35%") {}
        }
        div(class="text-2xl flex justify-center items-center mt-5 mb-20") {
            "당시 109달러였던 신비디아는 지금은 170달러를 넘었습니다!"
        }
        div(class="text-sm flex justify-center items-center mt-5") {
            "물론 중간에 팖, 만약 안 팔았다면 선물이 포켓몬이 아니라 스위치2가 됐을수도"
        }
    }
}

#[component]
fn Hover() -> View {
    view! {
        div(class="flex justify-center items-center mt-50") {
            div(class="card card-sm bg-base-200 max-w-60 shadow") {
                figure(class="hover-gallery") {
                    img(src="assets/sand.jpg") {}
                    img(src="assets/stand.jpg") {}
                    img(src="assets/chop.jpg") {}
                    img(src="assets/meat.jpg") {}
                    img(src="assets/study.jpg") {}
                }
                div(class="card-body") {
                    h2(class="card-title flex justify-between") {
                        "교황상의 레어한 모습들.zip"

                    }
                }
            }
        }
    }
}

#[component]
fn Collapse() -> View {
    view! {
        div(class="flex justify-center items-center mt-50") {
            div(tabindex="0", class="w-1/4 collapse bg-base-100 border-base-300 border") {
                div(class="collapse-title font-semibold") { "어쨌든 생일 축하해요! (클릭해보세요)" }
                div(class="collapse-content text-sm") {
                    div(class="flex justify-center items-center") {
                        img(src="assets/begom.jpg") {}
                    }
                    div(class="text-5xl flex justify-center items-center mt-5") {
                        "그럼요~"
                    }
                }
            }
        }
    }
}

#[component]
fn Love() -> View {
    view! {
        div(class="flex justify-center items-center mt-50") {
            div(class="text-5xl flex justify-center items-center mt-5") {
                "제 생일선물은 마음에 드셨나요? 마음에 드셨다면 별점을 눌러주세요~~"
                div(class="rating mt-5 gap-2") {
                    input(r#type="radio", name="rating-3", class="mask mask-heart bg-red-400", aria-label="1 star", checked=true) {}
                    input(r#type="radio", name="rating-3", class="mask mask-heart bg-red-400", aria-label="2 star") {}
                    input(r#type="radio", name="rating-3", class="mask mask-heart bg-red-400", aria-label="3 star") {}
                    input(r#type="radio", name="rating-3", class="mask mask-heart bg-red-400", aria-label="4 star") {}
                    input(r#type="radio", name="rating-3", class="mask mask-heart bg-red-400", aria-label="5 star") {}
                }
            }

        }
    }
}

#[component]
fn Lol() -> View {
    view! {
        div(class="flex justify-center items-center mt-100") {
            img(src="assets/lol.jpg") {}
        }
        div(class="text-3xl flex justify-center items-center mt-5 mb-50") {
            "알고 계셨나요?" br {} "이 사이트는 브라우저의 선호 설정에 맞추어 라이트/다크모드로 자동변경됩니다."
        }
    }
}

#[component]
fn App() -> View {
    view! {
        Talk()
        Loading()
        Trade()
        New9Kyo()
        Pokemon()
        Timer()
        Me2()
        Why()
        Squirrel()
        Hover()
        Collapse()
        Love()
        Lol()
    }
}

fn main() {
    sycamore::render(App);
}
