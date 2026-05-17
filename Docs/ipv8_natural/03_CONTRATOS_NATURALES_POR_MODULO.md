# Contratos naturales por modulo

## Que significa contrato en este contexto

Un contrato no es codigo.
Un contrato es una promesa clara entre dos partes del sistema.
Un contrato dice que informacion entra, que transformacion se espera, que informacion sale y que cosas no se permiten.

## Contrato entre identidad y plano de control

La capa de identidad entrega al plano de control una descripcion confiable del nodo.
El plano de control no deberia inventar identidad.
Solo evaluarla.

La identidad debe decir como se llama el nodo, que continuidad posee, que relaciones de confianza lo respaldan y que perfil operativo declara.
El plano de control usa esa informacion para decidir grupos, permisos, riesgos y politicas aplicables.

## Contrato entre plano de control y plano de datos

El plano de control nunca mueve paquetes.
Solo autoriza, limita o deniega condiciones de movimiento.

El plano de datos nunca debe redefinir politicas por capricho.
Puede adaptar tecnicamente una ruta, pero no deberia saltarse una regla soberana aprobada.

## Contrato entre discovery y rutas

Discovery entrega candidatos, contexto y salud observable.
Rutas decide que hacer con eso segun politicas, reputacion y objetivos de sesion.

Discovery no elige por si solo el camino final.
Rutas no deberia fingir que descubre el mundo sin apoyarse en discovery.

## Contrato entre observabilidad y todos los demas

Todo modulo importante debe producir estados observables.
Observabilidad no deberia necesitar adivinar el comportamiento del sistema.

Cada modulo le entrega a observabilidad eventos entendibles, no solo ruido.
Observabilidad devuelve vistas, resumenes y memoria operativa para humanos, dashboards y futuras automatizaciones.

## Contrato entre automatizacion y el resto del sistema

Automatizacion no crea soberania por si sola.
Automatizacion opera sobre permisos ya definidos.

Si un agente sugiere una reparacion, esa sugerencia debe quedar descrita, trazada y evaluada segun politicas.
Si un agente propone un cambio estructural, primero debe expresarlo como propuesta documental y no como sustitucion silenciosa del sistema.

## Contrato entre desktop y observabilidad

Desktop no deberia hablar con cada modulo interno por separado si puede evitarlo.
Desktop idealmente consume vistas estables de estado, resumen y accion permitida.

Eso evita que cada cambio interno rompa la experiencia de usuario.

## Contrato entre laboratorio y sistema canonico

Laboratorio existe para probar.
No para contaminar la definicion canonica.

Todo experimento nacido en laboratorio debe permanecer aislado hasta que una auditoria diga que merece ascender a un dominio estable.

## Contrato entre migraciones y arquitectura viva

Migraciones no son notas nostalgicas.
Son puentes concretos.

Cada migracion debe responder.
Que pieza vieja existe.
Que valor real tiene.
Que riesgo trae.
En que nueva carpeta deberia vivir.
Que cosas se descartan en vez de copiarse.

## Contrato entre benchmarks y decisiones

Los benchmarks no existen para decorar el proyecto.
Existen para frenar la fantasia.

Si una idea grandiosa empeora estabilidad, latencia, legibilidad o costo operativo, el benchmark debe tener autoridad para forzar una redefinicion del modulo.

## Contrato de nombres

Una palabra importante debe significar lo mismo en todos los documentos.
Si `sesion`, `ruta`, `peer`, `politica`, `nodo`, `seed`, `perfil` o `reputacion` cambian de significado entre dos archivos, la futura implementacion se deformara.

## Contrato de limite por archivo futuro

Cuando estos documentos se conviertan en codigo, cada archivo debera tener una responsabilidad nitida.
Si un archivo empieza a mezclar identidad, red, observabilidad, reparacion y UX, entonces no se esta respetando el contrato natural del sistema.

## Contrato de versionado

Cada contrato importante debe versionarse.
No porque se espere cambiarlo todo el tiempo.
Sino porque en proyectos grandes la ambiguedad entra cuando nadie recuerda desde cuando una palabra significa otra cosa.

## Conclusion contractual

Si en el futuro aparece una implementacion de `ipv8` que compile pero rompa estos contratos, entonces esa implementacion no sera una materializacion fiel del proyecto.
Sera solo una variacion tecnica sin fidelidad arquitectonica suficiente.
