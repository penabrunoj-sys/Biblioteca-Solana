use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod biblioteca {
    use super::*;

    // Crear biblioteca
    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let libros: Vec<Libro> = Vec::new();

        context.accounts.biblioteca.set_inner(Biblioteca {
            owner: owner_id,
            nombre,
            libros,
        });

        Ok(())
    }

    // Agregar libro con precio
    pub fn agregar_libro(
        context: Context<NuevoLibro>,
        nombre: String,
        paginas: u16,
        precio: u64,
    ) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libro = Libro {
            nombre,
            paginas,
            precio,
            disponible: true,
            comprador: None,
        };

        context.accounts.biblioteca.libros.push(libro);

        msg!("Libro agregado correctamente");

        Ok(())
    }

    // Eliminar libro
    pub fn eliminar_libro(context: Context<NuevoLibro>, nombre: String) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {
            if libros[i].nombre == nombre {
                libros.remove(i);
                msg!("Libro {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }

    // Ver libros
    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista de libros en la biblioteca: {:#?}",
            context.accounts.biblioteca.libros
        );

        Ok(())
    }

    // Alternar disponibilidad
    pub fn alternar_estado(context: Context<NuevoLibro>, nombre: String) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {

            if libros[i].nombre == nombre {

                let estado_actual = libros[i].disponible;
                libros[i].disponible = !estado_actual;

                msg!(
                    "El libro {} ahora tiene disponibilidad: {}",
                    nombre,
                    libros[i].disponible
                );

                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }

    // Comprar libro
    pub fn comprar_libro(context: Context<ComprarLibro>, nombre: String) -> Result<()> {

        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {

            if libros[i].nombre == nombre {

                require!(
                    libros[i].disponible == true,
                    Errores::LibroNoDisponible
                );

                libros[i].disponible = false;
                libros[i].comprador = Some(context.accounts.comprador.key());

                msg!("Libro comprado correctamente");

                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la biblioteca")]
    NoEresElOwner,

    #[msg("Error, el libro no existe")]
    LibroNoExiste,

    #[msg("Error, el libro no esta disponible")]
    LibroNoDisponible,
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    libros: Vec<Libro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {

    #[max_len(60)]
    nombre: String,

    paginas: u16,

    precio: u64,

    disponible: bool,

    comprador: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoLibro<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}

#[derive(Accounts)]
pub struct ComprarLibro<'info> {

    #[account(mut)]
    pub comprador: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}
