const INFO  = 0,
      WARN  = 1,
      ERROR = 2

const LEVELS = ['INFO ', 'WARN ', 'ERROR']

let level = INFO
let engine = () => {}

const log = (lvl, msg) => {
  if (lvl >= level) {
    let now = new Date()
    let yyyy = now.getYear()+1900,
          mm = now.getMonth()+1,
          dd = now.getDate(),
          HH = now.getHours(),
          MM = now.getMinutes(),
          SS = now.getSeconds()

    if (mm < 10) { mm = '0'+mm.toString() }
    if (dd < 10) { dd = '0'+dd.toString() }
    if (HH < 10) { HH = '0'+HH.toString() }
    if (MM < 10) { MM = '0'+MM.toString() }
    if (SS < 10) { SS = '0'+SS.toString() }

    engine(lvl, `${yyyy}-${mm}-${dd} ${HH}:${MM}:${SS} ${LEVELS[lvl]} ${msg}`)
  }
}

export default {
  INFO, WARN, ERROR,
  log,

  level(l) {
    if (typeof(l) !== 'undefined') {
      level = l
    }
    return level
  },

  via(fn) { engine = fn },

  info(msg)  { log(INFO,  msg) },
  warn(msg)  { log(WARN,  msg) },
  error(msg) { log(ERROR, msg) },

  console(l) {
    level = l
    engine = (_, msg) => console.log(msg)
  }
}
