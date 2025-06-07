# Rust-codes
Este repositorio es para proyectos fáciles o simples de Rust-lang

<br>

> Ejecución:
```
*Todos los proyectos deben compilarse y editarse con Cargo*.

Busca el proyecto que quieres ejecutar.
Busca el archivo "Cargo.toml" del proyecto.
En la consola o terminal, ejecuta "Cargo Update" en la misma ruta que el último archivo.
Ahora, ejecuta "Cargo build".
Y finalmente, ejecuta "Cargo Run".

El proyecto usa Rust + MongoDB. Falta un API para la carga de datos desde CSV a MongoDB.
Inicialmente, se cargaron los datos usando **Pentaho Data Integration**, la creación del API tiene que reemplazar esta carga manual por una API REST para que se pueda realizar la carga/actualización de datos a MongoDB.
```

> Consulta a las API:

```
Se ha probado el obtener la lista de empresas y sus datos. Usando la paginación y un limite de XX empresas por página.

```

1. Lista de empresas **get_empresas**, usando la paginación.
   http://localhost:8080/api/v1/empresas?page=1&limit=10

2. Obtiene los datos de una empresa por ruc. Usando la función **get_empresa_by_ruc**.
   http://localhost:8080/api/v1/empresa_ruc/{ruc}

3. Obtiene los datos de una empresa por ruc. Usando la función **get_empresa_by_razon_social**.
   http://localhost:8080/api/v1/empresa_razon_social/{razon_social}

4. La función **create_empresa** se utiliza para crear una nueva empresa.
5. La función **update_empresa_by_ruc** se utiliza para actualizar los datos de una empresa por su ruc.
6. La función **delete_empresa_por_ruc** se utiliza para eliminar una empresa por su ruc.

<br>

> NOTA:
> Algunos proyectos necesitan Parámetros del Sistema, estos parámetros se envían en el comando "Run" como "Cargo Run - Parámetro* <Parameter_1> <Parameter_2> ..."
> Las funciones de la 3-5 requieren parámetros aún no han sido probadas.