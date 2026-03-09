use anchor_lang::prelude::*;

declare_id!("    "); 

#[program]
pub mod taskmaster {
    use super::*;

    /// Crea una nueva tarea vinculada al usuario.
    pub fn create_task(ctx: Context<CreateTask>, id: u64, description: String) -> Result<()> {
        // Validamos que la descripción no esté vacía y no sea demasiado larga
        require!(description.chars().count() > 0, TaskError::EmptyDescription);
        require!(description.chars().count() <= 50, TaskError::DescriptionTooLong);

        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.owner.key();
        task.description = description;
        task.is_done = false;
        task.created_at = Clock::get()?.unix_timestamp;
        task.id = id;
        task.bump = ctx.bumps.task;

        msg!("TASKMASTER: Tarea #{} creada para {}", id, task.owner);
        Ok(())
    }

    /// Cambia el estado de la tarea (Completada/Pendiente).
    pub fn toggle_task(ctx: Context<UpdateTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.is_done = !task.is_done;

        msg!("TASKMASTER: Tarea #{} marcada como done={}", task.id, task.is_done);
        Ok(())
    }

    /// Elimina la tarea y devuelve los fondos (SOL) al dueño.
    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        msg!("TASKMASTER: Tarea eliminada. Renta recuperada.");
        Ok(())
    }
}

// ---------------- ESTRUCTURA DE DATOS ----------------



#[account]
pub struct Task {
    pub owner: Pubkey,       // 32 bytes
    pub id: u64,             // 8 bytes (ID único para la PDA)
    pub description: String, // 4 + 200 (máx 50 chars * 4 bytes)
    pub is_done: bool,       // 1 byte
    pub created_at: i64,     // 8 bytes
    pub bump: u8,            // 1 byte
}

impl Task {
    // Discriminador (8) + Owner (32) + ID (8) + String (4 + 200) + Done (1) + Date (8) + Bump (1)
    pub const LEN: usize = 8 + 32 + 8 + (4 + 200) + 1 + 8 + 1;
}

// ---------------- CONTEXTOS (PDAs) ----------------

#[derive(Accounts)]
#[instruction(id: u64)] // Pasamos el ID para usarlo en el seed
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = owner,
        space = Task::LEN,
        seeds = [b"task", owner.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        has_one = owner @ TaskError::Unauthorized,
        seeds = [b"task", owner.key().as_ref(), task.id.to_le_bytes().as_ref()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        close = owner, // Cierra la cuenta y devuelve el SOL al owner
        has_one = owner @ TaskError::Unauthorized,
        seeds = [b"task", owner.key().as_ref(), task.id.to_le_bytes().as_ref()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

// ---------------- ERRORES ----------------

#[error_code]
pub enum TaskError {
    #[msg("No tienes permisos para modificar esta tarea.")]
    Unauthorized,
    #[msg("La descripción es demasiado larga (máx 50 caracteres).")]
    DescriptionTooLong,
    #[msg("La descripción no puede estar vacía.")]
    EmptyDescription,
}
