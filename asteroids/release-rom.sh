cargo build --release

arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/asteroids target/asteroids.gba
gbafix -p -thello -cHELO -mRS target/asteroids.gba
mv target/asteroids.gba rom/
