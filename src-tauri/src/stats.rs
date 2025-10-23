use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{Local, Datelike, Duration, Weekday};

const STATS_FILE: &str = "stats.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub trigger_count: u32,
}

impl DailyStats {
    fn new(date: String) -> Self {
        Self {
            date,
            trigger_count: 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TodayStatsResponse {
    pub date: String,
    pub trigger_count: u32,
}

#[derive(Debug, Serialize)]
pub struct WeekStatsResponse {
    pub week_label: String,
    pub week_start: String,
    pub week_end: String,
    pub days: Vec<Option<DailyStats>>,
    pub can_go_next: bool,
    pub can_go_prev: bool,
}

pub struct StatsManager {
    stats_file: PathBuf,
    data: HashMap<String, DailyStats>,
    current_date: String,
}

impl StatsManager {
    pub fn new() -> Result<Self, String> {
        let home = std::env::var("HOME").map_err(|_| "无法获取用户目录".to_string())?;
        let stats_dir = PathBuf::from(format!("{}/.nopickie", home));
        
        // 确保目录存在
        fs::create_dir_all(&stats_dir).map_err(|e| format!("创建统计目录失败: {}", e))?;
        
        let stats_file = stats_dir.join(STATS_FILE);
        let data = Self::load_data(&stats_file)?;
        let current_date = Local::now().format("%Y-%m-%d").to_string();

        println!("📊 统计管理器已初始化: {:?}", stats_file);
        
        Ok(Self {
            stats_file,
            data,
            current_date,
        })
    }

    fn load_data(path: &PathBuf) -> Result<HashMap<String, DailyStats>, String> {
        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("读取统计文件失败: {}", e))?;
            let data: HashMap<String, DailyStats> = serde_json::from_str(&content)
                .map_err(|e| format!("解析统计文件失败: {}", e))?;
            println!("📂 加载统计数据: {} 天", data.len());
            Ok(data)
        } else {
            println!("📂 统计文件不存在，创建新文件");
            Ok(HashMap::new())
        }
    }

    fn save_data(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("序列化统计数据失败: {}", e))?;
        fs::write(&self.stats_file, content)
            .map_err(|e| format!("保存统计文件失败: {}", e))?;
        println!("💾 统计数据已保存");
        Ok(())
    }

    pub fn add_trigger(&mut self) -> Result<(), String> {
        self.check_date_change();
        
        let today = self.data.entry(self.current_date.clone())
            .or_insert_with(|| DailyStats::new(self.current_date.clone()));
        
        today.trigger_count += 1;
        
        println!("📈 今日触发次数: {}", today.trigger_count);
        
        self.save_data()?;
        Ok(())
    }

    pub fn get_today_stats(&self) -> TodayStatsResponse {
        let today = self.data.get(&self.current_date);
        
        if let Some(stats) = today {
            TodayStatsResponse {
                date: stats.date.clone(),
                trigger_count: stats.trigger_count,
            }
        } else {
            TodayStatsResponse {
                date: self.current_date.clone(),
                trigger_count: 0,
            }
        }
    }

    pub fn get_week_stats(&self, week_offset: i32) -> Result<WeekStatsResponse, String> {
        if week_offset > 0 {
            return Err("不能查看未来的周".to_string());
        }
        if week_offset < -11 {
            return Err("最多回溯12周".to_string());
        }

        let today = Local::now();
        let days_since_monday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(days_since_monday as i64 + (week_offset.abs() as i64 * 7));
        let sunday = monday + Duration::days(6);

        let week_start = monday.format("%Y-%m-%d").to_string();
        let week_end = sunday.format("%Y-%m-%d").to_string();

        let week_label = if week_offset == 0 {
            "本周".to_string()
        } else if week_offset == -1 {
            "上周".to_string()
        } else {
            format!("{}周前", week_offset.abs())
        };

        // 收集7天数据，但只返回昨天及之前的数据
        let mut days = Vec::new();
        let today_str = Local::now().format("%Y-%m-%d").to_string();
        
        for i in 0..7 {
            let date = monday + Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();
            
            // 如果是今天或未来的日期，跳过（不添加到数组）
            if date_str >= today_str {
                days.push(None);
            } else if let Some(stats) = self.data.get(&date_str) {
                days.push(Some(stats.clone()));
            } else {
                days.push(None);
            }
        }

        Ok(WeekStatsResponse {
            week_label,
            week_start,
            week_end,
            days,
            can_go_next: week_offset < 0,
            can_go_prev: week_offset > -11,
        })
    }

    fn check_date_change(&mut self) {
        let now = Local::now().format("%Y-%m-%d").to_string();
        if now != self.current_date {
            println!("📅 日期变更: {} -> {}", self.current_date, now);
            println!("📅 前一天数据已进入历史，今天从0开始");
            self.current_date = now;
        }
    }
}

