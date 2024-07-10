cargo build --release

arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/snake target/snake.gba
gbafix -p -thello -cHELO -mRS target/snake.gba
mv target/snake.gba rom/
