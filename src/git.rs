use git2::{Repository, StatusOptions, Status};
use ansi_term::Colour;
use std::fmt;
use eyre::Result;

#[derive(Debug)]
pub struct GitInfo {
    pub name: String,
    pub unstaged: u32,
    pub staged: u32,
    pub exists: bool,
}

impl Default for GitInfo {
    fn default() -> GitInfo {
        GitInfo{name:String::from(""), unstaged:0, staged:0, exists:false}
    }

}

impl fmt::Display for GitInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.exists {
            return Ok(())
        }

        // remote: 🛰️
        
        let fmt = Colour::Yellow.bold();
        write!(f, "on {} {}", fmt.paint(""), fmt.paint(&self.name))?;

        // TODO (2021-05-09): Maybe a match expression is nice?
        if self.unstaged > 0 {
            write!(f, " 🔴{}", self.unstaged)?;
        }

        if self.staged > 0 {
            if self.unstaged == 0 {
                write!(f, " 🚀")?;
            } else {
                write!(f, " 🌟{}", self.staged)?;
            }
        }
        Ok(())
            
    }
}

impl GitInfo {
    pub fn from(directory: &str) -> Result<GitInfo> {

        let repository = Repository::discover(directory)?;
        let name = match repository.head() {
            Ok(reference) => reference.shorthand().unwrap_or_default().into(),
            Err(_) => "-new- master".into(),
        };

        let ref mut status_filter = StatusOptions::new();
        status_filter
            .include_ignored(false)
            .include_unmodified(false)
            .include_untracked(true)
            .recurse_untracked_dirs(true);
        let stats = repository.statuses(Some(status_filter));

        // TODO: convert to binary and then make constant
        let mut wt_stats = Status::empty();
        wt_stats.toggle(Status::WT_DELETED);
        wt_stats.toggle(Status::WT_MODIFIED);
        wt_stats.toggle(Status::WT_NEW);
        wt_stats.toggle(Status::WT_RENAMED);
        wt_stats.toggle(Status::WT_TYPECHANGE);

        // TODO: convert to binary and then make constant
        let mut index_stats = Status::empty();
        index_stats.toggle(Status::INDEX_DELETED);
        index_stats.toggle(Status::INDEX_MODIFIED);
        index_stats.toggle(Status::INDEX_NEW);
        index_stats.toggle(Status::INDEX_RENAMED);
        index_stats.toggle(Status::INDEX_TYPECHANGE);

        let mut unstaged = 0;
        let mut staged = 0;
        if stats.is_ok() { // TODO: find more elegant way to do this
            for stat_entry in stats?.iter() {
                if stat_entry.status().intersects(wt_stats) {
                    unstaged = unstaged + 1;
                } 
                if stat_entry.status().intersects(index_stats) {
                    staged = staged + 1;
                }
            }
        }
        Ok(GitInfo{ name, staged, unstaged, exists:true})
    }
}
