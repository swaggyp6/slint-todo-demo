use std::rc::Rc;
use std::time::Duration;

use slint::Model;
use slint::ModelRc;
use slint::PlatformError;
use slint::SharedString;
use slint::Timer;
use slint::VecModel;
slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let app = App::new().unwrap();
    let app_handler = app.as_weak();
    let datetime_set = app.as_weak();
    let clear_hanlder = app.as_weak();
    let tmr = Timer::default();
    // let doday = chrono::Local::now().date_naive().to_string();
    // let tm = chrono::Local::now().time().to_string();
    tmr.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            datetime_set
                .unwrap()
                .set_cur_date(chrono::Local::now().date_naive().to_string().into());
            datetime_set.unwrap().set_cur_time(
                chrono::Local::now()
                    .time()
                    .format("%H:%M:%S")
                    .to_string()
                    .into(),
            );
        },
    );

    app_handler.unwrap().on_addtodo(move |input| {
        let model = app_handler.unwrap().get_todo_lists();
        let vecs = model.as_any().downcast_ref::<VecModel<ListMaps>>();
        let mut last_id = 0;
        match vecs {
            Some(e) => {
                // let ed = e.clone();

                for i in e.iter() {
                    last_id = i.id;
                }
                e.push(ListMaps { id: last_id + 1, item: input });
            }
            None => {
                let mds = Rc::new(VecModel::from(vec![ListMaps{id: 0, item: input}]));
                app_handler.unwrap().set_todo_lists(mds.into());
            }
        }
    });

    clear_hanlder.unwrap().on_cleal_all(move ||{
        let model = clear_hanlder.unwrap().get_todo_lists();
        let models = model.as_any().downcast_ref::<VecModel<ListMaps>>();
        if let Some(e) = models {
            e.clear();
        }
    });
    app.run()
}
