//use iced::theme::{Custom, Palette};

use iced::{executor, Color, Point};
use iced::{Element, Application,window, Settings,Theme,Command,Size};
use iced::widget::{Button, Column, Container, Row, Text};
use chrono::{Datelike, Local, NaiveDate};
use iced::window::{Position::Specific,Level,settings::PlatformSpecific};


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
    PrevYear,
    NextYear,
    DateSelected(NaiveDate),
}



fn main() -> iced::Result {
    let scale = 1.0;
    let height = 250.0*scale;
    let width = 400.0*scale;
    
    let x =960.0;
    let y = 480.0;
    
    let settings = Settings {           
        window: window::Settings {
            size: (Size::new(width,height)),
            exit_on_close_request: false,
            transparent: true, 
            level: Level::AlwaysOnTop,
            decorations: false,
            platform_specific: PlatformSpecific{skip_taskbar:true,
                ..Default::default()
            }, 
            visible: true,
            resizable: false,
            position: Specific(Point::new(x, y)),
            
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };
    CalendarApp::run(settings)
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
            Message::PrevYear => {
                let year = self.selected_date.year()-1;
                let mut month = 1;
                if year == self.today.year() { month = self.today.month();} 
                self.selected_date = NaiveDate::from_ymd_opt(year, month, 1).expect("err");
            }
            Message::NextYear => {
                let year = self.selected_date.year()+1;
                let mut month = 1;
                if year == self.today.year() { month = self.today.month();} 
                self.selected_date = NaiveDate::from_ymd_opt(year, month, 1).expect("err");
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
       // custom_theme::NordTheme::default()
    }


    fn view(&self) -> Element<Message> {
        let month = self.selected_date.format("%B %Y").to_string();
        let days = ["MO", "TU", "WE", "TH", "FR", "SA","SU"];
        let button_size = 25;
       
        let mut main_content_frame = Row::new();

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
            let mut week_row = Row::new().align_items(iced::Alignment::End);
             
            for _day in 0..7 {
                
                let month_date = date.day().to_string();
                
                let mut month_button_col:Vec<f32> = vec![0.0;3];
                if date == self.today {month_button_col[0]= 1.0;} 
                else if date.month() == self.selected_date.month() {month_button_col = vec![0.0, 0.0, 0.0];} 
                else {month_button_col = vec![1.0, 1.0,1.0];}

                let date_label = Button::new( Text::new(month_date)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(button_size) 
                                    .style(iced::theme::Text::Color(Color::from_rgb(month_button_col[0],
                                        month_button_col[1], 
                                        month_button_col[2]))))
                                    .on_press(Message::DateSelected(date));
                                    
                week_row = week_row.push(date_label);
                date = date + chrono::Duration::days(1);
            }
            calender_frame = calender_frame.push(week_row);
        }
        let mut calender_row = Row::new().push(calender_frame);
        
        let mut big_week_col = Column::new();
        let year = self.selected_date.format("%Y").to_string();
        let year_frame = Column::new()
            .push(
                Row::new().align_items(iced::Alignment::Center)
                    .push(Button::new( Text::new("<")).on_press(Message::PrevYear))
                    .push(Text::new(year)
                    .horizontal_alignment(iced::alignment::Horizontal::Center))
                    
                    
                    .push(Button::new( Text::new(">")).on_press(Message::NextYear)),
            );
        big_week_col = big_week_col.push(year_frame);
        
        date = self.selected_date - chrono::Duration::days((self.selected_date.weekday().number_from_monday() - 1) as i64);
        println!("Monday :{:?}",date);
    
        
        for _day in 0..7 {
            let week_day = date.format("%A").to_string();
            let week_date =  date.format("%-d, ").to_string() + &week_day[0..3];
            let mut week_button_col:Vec<f32> = vec![0.0;3];
           
            if date == self.today {week_button_col[0]= 1.0;} 
            else if date.month() == self.selected_date.month() {week_button_col = vec![0.0, 0.0, 0.0];} 
            else {week_button_col = vec![1.0, 1.0,1.0];}
            
            let week_label = Button::new( Text::new(week_date)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            //.vertical_alignment(iced::alignment::Vertical::Center)
                            .width(button_size*5) 
                            .style(iced::theme::Text::Color(Color::from_rgb(week_button_col[0],
                                                                            week_button_col[1], 
                                                                            week_button_col[2]))))
                            .on_press(Message::DateSelected(date));
            
            big_week_col = big_week_col.push(week_label);
            date = date + chrono::Duration::days(1);
        }
        
        calender_row = calender_row.push(big_week_col);
        
        
        main_content_frame = main_content_frame.push(calender_row);  
        
        Container::new(main_content_frame).center_x().center_y().into()
    }
}

