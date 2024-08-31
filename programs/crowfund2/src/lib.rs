use anchor_lang::prelude::*;
//use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program;
declare_id!("GKyC3AKQt3TgW2nEJaZYUXXiGBjBMsURgsqvJRooZgzr");

#[program]
pub mod cagnotte2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        let cagnotte = &mut ctx.accounts.cagnotte;
        cagnotte.owner = *ctx.accounts.user.key;
        cagnotte.name = name.as_bytes().to_vec();
        cagnotte.amount = 0;
    // intisalise la cagnotte  avec un montant de 0 PDA a init avec une seed qui attend le nom (string) cagnotte, la clé publique de 
    // la clé publique de l'utilisateur et la string du nom de la cagnotte.  
        Ok(())
    }

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        // Vérifier si le compte admin existe déjà donc si le solde est a 0 :)
        
        /*if ctx.accounts.admin_account.to_account_info().lamports() > 0 {
            return Err(ErrorCode::AdminAccountAlreadyExists.into());
        }*/

        let admin_account = &mut ctx.accounts.admin_account;
        // seul l'init du program devient le premier admin 
        admin_account.admins = vec![*ctx.accounts.user.key];

        Ok(())
    }

    pub fn add_admin(ctx: Context<AdminManagement>, new_admin: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;

        // test si admin = le demandeur
        
        if !admin_account.admins.contains(ctx.accounts.user.key) {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Ajouter le nouvel admin si ce n'est pas déjà un admin
        if !admin_account.admins.contains(&new_admin) {
            admin_account.admins.push(new_admin);
            msg!("Admin ajouté: {}", new_admin);
        } else {
            msg!("Admin {} est déjà dans la liste", new_admin);
        }

        Ok(())
    }
    pub fn revoke_admin(ctx: Context<AdminManagement>, admin_to_revoke: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;

        // Vérifier que l'utilisateur actuel est déjà admin
        if !admin_account.admins.contains(ctx.accounts.user.key) {
            return Err(ErrorCode::Unauthorized.into());
        }

        let mut admin_found = false;

        // Parcourir la liste des admins et révoquer celui qui correspond à admin_to_revoke
        for i in 0..admin_account.admins.len() {
            if admin_account.admins[i] == admin_to_revoke {
                admin_account.admins.remove(i);
                admin_found = true;
                msg!("Admin révoqué: {}", admin_to_revoke);
                break;
            }
        }

        if !admin_found {
            msg!("Admin {} n'est pas trouvé", admin_to_revoke);
        }

        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let cagnotte = &mut ctx.accounts.cagnotte;


        if cagnotte.locked {
        return Err(ErrorCode::CagnotteLocked.into());
    }


        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.cagnotte.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        ctx.accounts.cagnotte.amount += amount;
        ctx.accounts.contribution.amount +=amount;
        Ok(())
    }

        // l'appel a contribute, crée un pda si necessaire, de la forme suivante : 
        //seeds= [b"contribution", cagnotte.key().as_ref(), user.key().as_ref()],
        //donc l'appel a contribution se fait par la seed string contribution, 
        // la public key du pda de la cagnotte que l'on appelle  public key de l'user, 



    pub fn get_balance(ctx: Context<GetBalance>) -> Result<()> {
        let cagnotte = &ctx.accounts.cagnotte;
        msg!(
            "The current balance of the cagnotte is: {} lamports",
            cagnotte.amount
        );
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let cagnotte = &mut ctx.accounts.cagnotte;
        msg!("account to withdraw {} ", cagnotte.key());
        // Vérifier si la cagnotte est verrouillée
        if cagnotte.locked {
        return Err(ErrorCode::CagnotteLocked.into());
        }
        msg!("amount to withdraw {} Lamports", amount);
        // msg!("account to withdraw {} ", account_address);
        
        //verification du owner pour s'assurer que personne d'autre ne puisse recuperer les fonds. 
        if cagnotte.owner != *ctx.accounts.user.key {
            return Err(ErrorCode::Unauthorized.into());
        }
        
        // verification du montant que  la somme detenue par le pda est au superieur ou égal au montant du amount 
        if cagnotte.amount < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        cagnotte.amount -= amount;
        **ctx
            .accounts
            .cagnotte
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;

        Ok(())
    }
    pub fn lock_cagnotte(ctx: Context<ManageCagnotteLock>) -> Result<()> {
    let cagnotte = &mut ctx.accounts.cagnotte;

    let admin_account = &ctx.accounts.admin_account;

    // Vérifier si l'utilisateur est bien un administrateur
    if !admin_account.admins.contains(ctx.accounts.user.key) {
        return Err(ErrorCode::Unauthorized.into());
    }
    cagnotte.locked = true;
    msg!("La cagnotte {} a été verrouillée.", String::from_utf8_lossy(&cagnotte.name));

    Ok(())
}

pub fn unlock_cagnotte(ctx: Context<ManageCagnotteLock>) -> Result<()> {
    let cagnotte = &mut ctx.accounts.cagnotte;
    let admin_account = &ctx.accounts.admin_account;

    // Vérifier si l'utilisateur est bien un administrateur
    if !admin_account.admins.contains(ctx.accounts.user.key) {
        return Err(ErrorCode::Unauthorized.into());
    }
    
    

    cagnotte.locked = false;
    msg!("La cagnotte {} a été déverrouillée.", String::from_utf8_lossy(&cagnotte.name));

    Ok(())
}

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 4 + 64 + 1,  
        seeds = [b"cagnotte", user.key().as_ref(), name.as_bytes()], 
        bump
    )]
    pub cagnotte: Account<'info, Cagnotte>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub cagnotte: Account<'info, Cagnotte>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        payer=user,
        space = 8+32+8, 
        seeds= [b"contribution", cagnotte.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, Contribution>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub cagnotte: Account<'info, Cagnotte>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    pub cagnotte: Account<'info, Cagnotte>,
}


#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 * 10, // Taille pour jusqu'à 10 admins (ajustez selon vos besoins)
        seeds = [b"admin-account"],
        bump
    )]
    pub admin_account: Account<'info, AdminAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminManagement<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, AdminAccount>,
    #[account(mut)]
    pub user: Signer<'info>, // Doit être un admin pour ajouter/révoquer
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct ManageCagnotteLock<'info> {
    #[account(mut)]
    pub cagnotte: Account<'info, Cagnotte>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub admin_account: Account<'info, AdminAccount>,  // Ajout du compte admin pour la vérification
}

#[account]
pub struct Cagnotte {
    pub owner: Pubkey,
    pub name: Vec<u8>,
    pub amount: u64,
    pub locked: bool,
    pub contributions: Vec<(Pubkey, u64)>,
}

//compte Contribution pour suivre l'avancement de chaque user dans chaque cagnotte 
#[account]
pub struct Contribution {
    pub user: Pubkey,
    pub amount: u64,
    
    
}

//compte  des admins 
#[account]
pub struct AdminAccount {
    pub admins: Vec<Pubkey>, // Liste des admins
}



//gestion des erreurs 
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Insufficient funds in the cagnotte.")]
    InsufficientFunds,
    #[msg("Admin account already exists.")]
    AdminAccountAlreadyExists,
    #[msg("The cagnotte is currently locked.")]
    CagnotteLocked,
}
