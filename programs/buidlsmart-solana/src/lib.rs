use anchor_lang::prelude::*;

declare_id!("56rhhZsNu5WZ7uxqxG8LFp5gp9mgJnb2CrYaVuJa81WV");

#[program]
pub mod buidlsmart_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn new_application(
        ctx: Context<NewApplication>,
        application_name: String,
        current_timestamp: String,
    ) -> Result<()> {
        let base_app = &mut ctx.accounts.base_application;
        let user = &mut ctx.accounts.user;

        let app = Single {
            id: base_app.application_list.len() as u32,
            name: application_name,
            creator: *user.to_account_info().key,
            objects: vec![],
            functions: vec![],
            timestamp: current_timestamp,
        };
        base_app.application_list.push(app);
        Ok(())
    }

    pub fn delete_application(ctx: Context<NewApplication>, id: u32) -> Result<()> {
        let base_app = &mut ctx.accounts.base_application;
        let index = base_app
            .application_list
            .iter()
            .position(|x| x.id == id)
            .unwrap();
        base_app.application_list.remove(index);
        Ok(())
    }

    pub fn add_object_to_application(
        ctx: Context<NewApplication>,
        app_id: u32,
        object_name: String,
        field_names: Vec<String>,
        field_types: Vec<String>,
    ) -> Result<()> {
        let base_app = &mut ctx.accounts.base_application;

        let app = &mut base_app.application_list.get_mut(app_id as usize).unwrap();

        let mut i = 0;
        let mut fields = vec![];
        while i < field_names.len() {
            let f = Field {
                id: i as u32,
                name: field_names.get(i).unwrap().to_string(),
                field_type: field_types.get(i).unwrap().to_string(),
            };
            fields.push(f);

            i += 1;
        }

        let object = Object {
            id: app.objects.len() as u32,
            name: object_name,
            fields,
        };
        let objects = &mut app.objects;
        {
            objects.push(object);
        }
        Ok(())
    }

    pub fn add_function_to_application(
        ctx: Context<AddFunctionToApplication>,
        app_id: u32,
        function_name: String,
        function_action: String,
        target_id: u32,
        param_name: Vec<String>,
        param_type: Vec<String>,
    ) -> Result<()> {
        let base_app = &mut ctx.accounts.base_application;
        let user = &mut ctx.accounts.user;

        let app = &mut base_app.application_list.get_mut(app_id as usize).unwrap();

        let mut i = 0;
        let mut params = vec![];
        while i < param_name.len() {
            let p = Parameter {
                id: i as u32,
                name: param_name.get(i).unwrap().to_string(),
                param_type: param_type.get(i).unwrap().to_string(),
            };
            params.push(p);

            i += 1;
        }

        let function = Function {
            id: app.functions.len() as u32,
            name: function_name,
            target: app.objects.get(target_id as usize).unwrap().clone(),
            action: function_action,
            creator: *user.to_account_info().key,
            parameters: params,
        };
        let functions = &mut app.functions;
        {
            functions.push(function);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_application: Account<'info, Application>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewApplication<'info> {
    #[account(mut)]
    pub base_application: Account<'info, Application>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddFunctionToApplication<'info> {
    #[account(mut)]
    pub base_application: Account<'info, Application>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Application {
    pub application_list: Vec<Single>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Single {
    pub id: u32,
    pub name: String,
    pub objects: Vec<Object>,
    pub functions: Vec<Function>,
    pub creator: Pubkey,
    pub timestamp: String,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Field {
    pub id: u32,
    pub name: String,
    pub field_type: String,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Function {
    pub id: u32,
    pub name: String,
    pub action: String,
    pub creator: Pubkey,
    pub parameters: Vec<Parameter>,
    pub target: Object,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Parameter {
    pub id: u32,
    pub name: String,
    pub param_type: String,
}
