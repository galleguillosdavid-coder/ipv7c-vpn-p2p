# Paquete natural de ipv8

Este directorio no contiene codigo.
Este directorio no contiene crates.
Este directorio no contiene esqueletos compilables.
Este directorio existe para que `ipv8` pueda pensarse, repartirse y construirse sin obligar a una sola inteligencia artificial a cargar todo el proyecto a la vez.

La idea central es simple.
Primero se fija el lenguaje del sistema en forma humana.
Despues se divide el sistema en dominios comprensibles.
Despues se describe cada carpeta y cada archivo como una responsabilidad.
Despues se fijan los contratos entre piezas.
Y solo al final, en otra fase, una o varias IAs convierten cada pieza en codigo usando estos documentos como fuente de verdad.

## Que problema resuelve este paquete

El proyecto es demasiado ambicioso para una sola sesion larga de implementacion ciega.
Si se intenta convertir toda la vision a codigo de una sola vez, la IA mezcla capas, repite conceptos, inventa dependencias, cambia nombres y termina colapsando el orden interno del sistema.

Por eso este paquete hace cinco cosas.

Primero, reduce la ambicion total a modulos narrables.
Segundo, transforma ideas abstractas en responsabilidades concretas.
Tercero, define fronteras entre carpetas, archivos y decisiones.
Cuarto, permite construir por lotes con varias IAs sin que cada una tenga que imaginar el sistema completo.
Quinto, conserva la soberania del diseño en documentos humanos antes de bajar a implementacion.

## Como leer este directorio

Empieza por `01_ARQUITECTURA_NATURAL_DE_IPV8.md`.
Despues pasa a `02_ARBOL_NATURAL_DE_CARPETAS_Y_ARCHIVOS.md`.
Luego revisa `03_CONTRATOS_NATURALES_POR_MODULO.md`.
Finalmente usa `04_GUIA_DE_TRANSFORMACION_A_CODIGO_CON_VARIAS_IAS.md` cuando quieras repartir el trabajo entre varias IAs.

## Regla principal

Mientras este paquete siga siendo la base de `ipv8`, ninguna IA deberia inventar carpetas, archivos o contratos que contradigan estos documentos sin dejar primero una propuesta textual de cambio.

## Regla de modularidad

Cada modulo de `ipv8` debe poder describirse de forma humana en una sola pagina clara.
Si un modulo no puede explicarse con claridad en lenguaje natural, entonces todavia no esta listo para convertirse en codigo.

## Regla de soberania documental

La documentacion natural manda antes que el codigo nuevo.
No porque el codigo valga menos.
Sino porque en un proyecto de esta escala el desorden nace mucho antes del compilador.
Nace cuando dos agentes entienden de forma distinta la misma palabra.

## Resultado buscado

Al terminar esta fase, `ipv8` deberia ser una ciudad dibujada con claridad antes de poner un solo ladrillo tecnico nuevo.
