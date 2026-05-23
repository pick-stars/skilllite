//! 进化反馈库、生长调度、待审核技能与 `run_evolution` / 后台 `evolution run` 触发（桌面 UI 数据源）。
//!
//! 子模块：`config`（合并后的环境与阈值）、`growth`（Life Pulse）、`status`（状态载荷）、
//! `pending`（待审核技能）、`authorize`（能力进化授权）、`backlog`（队列行）、`trigger`（CLI run）。

mod authorize;
mod backlog;
mod config;
mod growth;
mod pending;
mod status;
mod trigger;

pub use authorize::authorize_capability_evolution;
pub use backlog::{
    get_evolution_proposal_status, load_evolution_backlog, EvolutionBacklogRowDto,
    EvolutionProposalStatusDto,
};
pub use growth::evolution_growth_due;
pub use pending::{
    evolution_confirm_pending_skill, evolution_reject_pending_skill, list_evolution_pending_skills,
    read_evolution_pending_skill_md, PendingSkillDto,
};
pub use status::{load_evolution_status, EvolutionStatusPayload};
pub use trigger::trigger_evolution_run;
