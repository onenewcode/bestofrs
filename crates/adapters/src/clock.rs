use app::snapshot::Clock;

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn utc_today_ymd(&self) -> chrono::NaiveDate {
        chrono::Utc::now().date_naive()
    }

    fn utc_now_rfc3339(&self) -> String {
        chrono::Utc::now().to_rfc3339()
    }
}
