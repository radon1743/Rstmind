use iced::{executor, Length};
use iced::{Element, Application, Settings,Theme,Command};
use iced::widget::{Button, Container, Row,Column, Text,Space};
use chrono::{Datelike, Local, NaiveDate};

fn main() -> iced::Result {
    CalendarApp::run(Settings::default())
}

#[derive(Default)]
struct CalendarApp {
    selected_date: NaiveDate,
    //buttons: Vec<button::State>,
    today: NaiveDate,
    
    }

#[derive(Debug, Clone)]
enum Message {
    PrevMonth,
    NextMonth,
    DateSelected(NaiveDate),
    
}

impl Application for CalendarApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
	type Theme = Theme;
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                selected_date: Local::now().date_naive(),
                //buttons: vec![button::State::new(); 42], // 6 weeks * 7 days
                today: Local::now().date_naive(),
                
            },
            Command::none(),
        )
    }
    
    fn title(&self) -> String {
        String::from("Calendar")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PrevMonth => {
                let (year, month) = (self.selected_date.year(), self.selected_date.month());
                let new_month = if month == 1 { 12 } else { month - 1 };
                let new_year = if month == 1 { year - 1 } else { year };
                self.selected_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).expect("err");
            }
            Message::NextMonth => {
                let (year, month) = (self.selected_date.year(), self.selected_date.month());
                let new_month = if month == 12 { 1 } else { month + 1 };
                let new_year = if month == 12 { year + 1 } else { year };
                self.selected_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).expect("err");
            }
            Message::DateSelected(date) => {
                self.selected_date = date;    
                println!("date:{:?}",date);

            }   
      
        }
        Command::none()
    }
	fn theme(&self)-> Self::Theme{
		Theme::Nord
	}
    fn view(&self) -> Element<Message> {
        let month = self.selected_date.format("%B %Y").to_string();
        let days = ["MO", "TU", "WE", "TH", "FR", "SA","SU"];
        let button_size = 25;
       
        let mut main_content_frame = Column::new()
            .push(Space::new(Length::Fill,150));
            

        let mut calender_frame = Column::new()
            .push(
                Row::new().align_items(iced::Alignment::Center)
                    .push(Button::new( Text::new("<")).on_press(Message::PrevMonth))
                    
                    .push(Text::new(month)
                    .horizontal_alignment(iced::alignment::Horizontal::Center))
                    
                    
                    .push(Button::new( Text::new(">")).on_press(Message::NextMonth)),
            )
            .push(Row::with_children(days.iter().map(|&day| Text::new(day)
            .width(button_size+10)
            .horizontal_alignment(iced::alignment::Horizontal::Center).into())));
        
        

      
        let first_day_of_month:NaiveDate = NaiveDate::from_ymd_opt(self.selected_date.year(), self.selected_date.month(), 1).unwrap();
        let weekday = first_day_of_month.weekday().num_days_from_monday();
        
        let mut date = first_day_of_month - chrono::Duration::days(weekday as i64);
        

        for _week in 0..6 {
            let mut week_row = Row::new();
            
            for _day in 0..7 {
                let date_label = if date == self.today {
                    Button::new( Text::new(date.day().to_string())
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(button_size) 
                                    .style(iced::theme::Text::Color([1.0, 0.0, 0.0].into())))
                                    .on_press(Message::DateSelected(date))
                } 
                else if date.month() == self.selected_date.month() {
                    Button::new( Text::new(date.day().to_string()) 
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(button_size)
                                    .style(iced::theme::Text::Color([0.0, 0.0, 0.0].into())))
                                    .on_press(Message::DateSelected(date))                         
                } else {
                    Button::new( Text::new(date.day().to_string())
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)       
                                    .width(button_size)
                                    .style(iced::theme::Text::Color([1.0, 1.0,1.0].into())))
                                    .on_press(Message::DateSelected(date)) 
                };
                
                week_row = week_row.push(date_label);
                
                date = date + chrono::Duration::days(1);
            }
            calender_frame = calender_frame.push(week_row);
        }
        let mut calender_row = Row::new().push(calender_frame);
        
        
        date = first_day_of_month - chrono::Duration::days(weekday as i64);
        let mut big_week_row = Row::new().align_items(iced::Alignment::End);

        
        let first_day_of_week:NaiveDate = NaiveDate::from_ymd_opt(self.selected_date.year(), self.selected_date.month(), self.selected_date.day() - self.selected_date.weekday().num_days_from_monday()).unwrap();
        

        println!("{}",first_day_of_week);
        for _day in 0..7 {
            
            let week_label = if date == self.today {
                Button::new( Text::new(date.day().to_string())
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .width(button_size*3) 
                                .style(iced::theme::Text::Color([1.0, 0.0, 0.0].into())))
                                .on_press(Message::DateSelected(date))
            } 
            else if date.month() == self.selected_date.month() {
                 Button::new( Text::new(date.day().to_string()) 
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .width(button_size*3)
                                .style(iced::theme::Text::Color([0.0, 0.0, 0.0].into())))
                                .on_press(Message::DateSelected(date))                         
            } else {
                Button::new( Text::new(date.day().to_string())
                                .horizontal_alignment(iced::alignment::Horizontal::Center)       
                                .width(button_size*3)
                                .style(iced::theme::Text::Color([1.0, 1.0,1.0].into())))
                                .on_press(Message::DateSelected(date)) 
            };
                
            big_week_row = big_week_row.push(week_label);
             
            date = date + chrono::Duration::days(1);
        }
        
        calender_row = calender_row.push(big_week_row);
        main_content_frame = main_content_frame.push(calender_row);  
        
        Container::new(main_content_frame).center_x().center_y().into()
    }
}