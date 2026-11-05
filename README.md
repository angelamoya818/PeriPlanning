# PeriPlanning

## Definición del problema
Mi padre es perito de seguros autónomo desde hace muchos años. Todos los días, en su plataforma, se le añaden peritaciones en una tabla. En esa tabla están mezclados muchos tipos diferentes de peritaciones: diferentes estados (nuevo/reabierto), de diferentes aseguradoras, y que deben ser vistos de formas diferentes (videoperitación o presencial).

Hay una serie de condiciones que se deben cumplir:
1. Si el siniestro es de Meridiano, Santander, AXA empresas o Mapfre empresas, siempre debe ser visto presencialmente.
2. Si la cuantía aproximada de los daños supera los 4000€, debe ser visto presencialmente.
3. Para los siniestros de Mapfre empresas y AXA empresas, si el perito no contacta con el asegurado en un plazo de dos días, y no lo cierra en un plazo de 6 días desde el contacto con el asegurado, el perito sufre una penalización.
4. Los siniestros nuevos con más de 30 días sin haber sido vistos penalizan al perito.
5. Para todos los siniestros, si el asegurado no ha sido contactado en un plazo de 4 días, el perito es penalizado.

La penalización que se comenta, supone una disminución del 5% de la minuta que cobra el perito. Esta penalización es acumulable; es decir, si se duplica el tiempo (4 días para Mapfre y Axa, y 8 días para el resto de aseguradoras), se marca como bloqueado y se debe comunicar al supervisor que se ha contactado con el asegurado para que desbloquee el expediente; y si vuelve a pasar el plazo (total de 6 días para Mapfre y Axa, y 12 días para el resto de aseguradoras), se devuelve el expediente y se asigna a otro perito, perdiendo además la minuta asociada.

En resumen; **se requiere extraer, analizar y filtrar datos extraídos de la tabla** para poder solucionar el problema.

Suelen entrar alrededor de 15 o 20 expedientes nuevos por semana, por lo que el trabajo acumulado acaba volviendo muy caótico el mantener un orden y metodología.

## Conocimiento acerca del problema

Como ya he mencionado, mi padre lleva alrededor de 30 años trabajando de perito de seguros autónomo. Desde que yo era pequeño, de su jornada laboral, suele gastar aproximadamente la mitad en la organización por este sistema arcaico. Esto hace que se pierda mucho tiempo en la organización del trabajo, lo que es contraproducente porque al ser autónomo cobra cuantías por número y cuantía en cada peritación.

Al automatizar este proceso, la productividad aumenta mucho, lo que acaba resultando en mayor ganancia económica.

## Ejemplo de tabla

Los datos son censurados porque contienen información potencialmente sensible.

<img src="Media/Fotos/0_3_tablaPerito.jpg" width="100%">

## Configuración

* Licencia: GNU General Public License, versión 3.
* Control de versiones: Uso de git (mediante clave SSH censurada, véase captura abajo) aplicado a cualidades de desarrollo ágil (creación de ramas, issues, milestones, pipelines CI/CD, etc).
<img src="../../Media/Fotos/0_2_ssh.jpg" width="40%">

## Documentos

- Se puede consultar lo relativo al objetivo 0 [aquí](/Objetivos/Objetivo_0/objetivo-0.md).
- Se pueden consultar los perfiles de usuario [aquí](/docs/personas.md).
- Se pueden consultar las jornadas de usuario [aquí](/docs/jornadas_usuario.md).
- Se pueden consultar las historias de usuario [aquí](/docs/historias_de_usuario.md).
- Se pueden consultar los milestones [aquí](/docs/milestones.md).
